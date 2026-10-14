use std::ops::Range;

const MAX_IPMA: usize = 4;
const KNOWN_IPMA_BITS: u32 = 0x0100_0001;

/// The box starting at `pos` and ending by `end`
fn next_box(data: &[u8], pos: usize, end: usize) -> Option<([u8; 4], Range<usize>)> {
    let header = data.get(pos..(pos + 8).min(end))?.get(..8)?;
    let kind: [u8; 4] = header[4..].try_into().unwrap();
    let mut body = pos + 8;

    let box_end = match u32::from_be_bytes(header[..4].try_into().unwrap()) {
        0 => end,
        1 => {
            let large = data.get(body..(body + 8).min(end))?.get(..8)?;
            body += 8;
            let size = usize::try_from(u64::from_be_bytes(large.try_into().unwrap())).ok()?;
            pos.checked_add(size)?
        }
        size => pos.checked_add(size as usize)?,
    };
    if box_end > end || box_end < body {
        return None;
    }

    Some((kind, body..box_end))
}

fn children(
    data: &[u8],
    range: Range<usize>,
) -> impl Iterator<Item = ([u8; 4], Range<usize>)> + '_ {
    let mut pos = range.start;
    std::iter::from_fn(move || {
        let (kind, body) = next_box(data, pos, range.end)?;
        pos = body.end;
        Some((kind, body))
    })
}

fn find(data: &[u8], range: Range<usize>, kind: &[u8; 4]) -> Option<Range<usize>> {
    children(data, range)
        .find(|(k, _)| k == kind)
        .map(|(_, body)| body)
}

fn meta_body(data: &[u8]) -> Option<Range<usize>> {
    let meta = find(data, 0..data.len(), b"meta")?;
    (meta.len() >= 4).then(|| meta.start + 4..meta.end)
}

fn read_uint(data: &[u8], pos: usize, width: usize) -> Option<u32> {
    let bytes = data.get(pos..pos.checked_add(width)?)?;
    Some(bytes.iter().fold(0, |acc, &b| (acc << 8) | b as u32))
}

/// Sets the essential bit on 'clap', 'irot' and 'imir' associations, MIAF requires it and libavif
/// rejects the whole file without it
pub fn mark_transforms_essential(data: &mut [u8]) -> bool {
    let Some(iprp) = meta_body(data).and_then(|meta| find(data, meta, b"iprp")) else {
        return false;
    };

    let Some(ipco) = find(data, iprp.clone(), b"ipco") else {
        return false;
    };

    let transforms: Vec<bool> = children(data, ipco)
        .take(0x7FFF)
        .map(|(kind, _)| matches!(&kind, b"clap" | b"irot" | b"imir"))
        .collect();
    let is_transform = |index: u32| {
        (index as usize)
            .checked_sub(1)
            .and_then(|i| transforms.get(i))
            .is_some_and(|&transform| transform)
    };

    match ipma_pass(data, iprp.clone(), &is_transform, false) {
        None | Some(0) => false,
        Some(_) => ipma_pass(data, iprp, &is_transform, true).is_some(),
    }
}

/// Walks every ipma box in `iprp`
fn ipma_pass(
    data: &mut [u8],
    iprp: Range<usize>,
    is_transform: &impl Fn(u32) -> bool,
    patch: bool,
) -> Option<usize> {
    let mut found = 0;
    let mut pos = iprp.start;
    while let Some((kind, body)) = next_box(data, pos, iprp.end) {
        pos = body.end;
        if kind == *b"ipma" {
            found += walk_ipma(&mut data[body], is_transform, patch)?;
        }
    }
    Some(found)
}

/// Counts the non-essential associations `is_transform` picks in the body of one ipma box, and
/// with `patch` marks them essential. None if the box is malformed
fn walk_ipma(ipma: &mut [u8], is_transform: &impl Fn(u32) -> bool, patch: bool) -> Option<usize> {
    let version = *ipma.first()?;
    let index_bytes = if read_uint(ipma, 1, 3)? & 1 != 0 {
        2
    } else {
        1
    };
    let entries = read_uint(ipma, 4, 4)?;
    let mut pos = 8;
    let mut found = 0;

    for _ in 0..entries {
        pos += if version < 1 { 2 } else { 4 };
        let count = *ipma.get(pos)?;
        pos += 1;
        for _ in 0..count {
            let value = read_uint(ipma, pos, index_bytes)?;
            if value >> (index_bytes * 8 - 1) == 0 && is_transform(value) {
                found += 1;
                if patch {
                    ipma[pos] |= 0x80;
                }
            }
            pos += index_bytes;
        }
    }
    Some(found)
}

/// Points 'auxl' references to the missing item 0 at the primary item instead, the only image an
/// auxiliary item can belong to when the reference is broken
pub fn fix_aux_reference_to_item_zero(data: &mut [u8]) -> bool {
    let Some(meta) = meta_body(data) else {
        return false;
    };
    let (Some(pitm), Some(iref)) = (find(data, meta.clone(), b"pitm"), find(data, meta, b"iref"))
    else {
        return false;
    };

    let pitm_body = &data[pitm];
    let primary = match pitm_body.first() {
        Some(0) => read_uint(pitm_body, 4, 2),
        Some(1) => read_uint(pitm_body, 4, 4),
        _ => None,
    };
    let (Some(primary), Some(&version)) = (primary, data[iref.clone()].first()) else {
        return false;
    };
    let width = if version == 0 { 2 } else { 4 };
    if primary == 0 || (width == 2 && primary > u16::MAX as u32) {
        return false;
    }

    match iref_pass(data, iref.clone(), width, primary, false) {
        None | Some(0) => false,
        Some(_) => iref_pass(data, iref, width, primary, true).is_some(),
    }
}

/// Walks every reference box in the iref body
fn iref_pass(
    data: &mut [u8],
    iref: Range<usize>,
    width: usize,
    primary: u32,
    patch: bool,
) -> Option<usize> {
    let mut found = 0;
    let mut pos = iref.start + 4;
    while let Some((kind, body)) = next_box(data, pos, iref.end) {
        pos = body.end;
        found += walk_reference(&mut data[body], kind, width, primary, patch)?;
    }
    Some(found)
}

/// Counts the item 0 targets of one 'auxl' reference from an item other than `primary`, and with
/// `patch` points them at `primary`. None if the reference is malformed
fn walk_reference(
    reference: &mut [u8],
    kind: [u8; 4],
    width: usize,
    primary: u32,
    patch: bool,
) -> Option<usize> {
    let from = read_uint(reference, 0, width)?;
    let count = read_uint(reference, width, 2)? as usize;
    let ids = width + 2;
    if ids + count * width > reference.len() {
        return None;
    }
    if kind != *b"auxl" || from == primary {
        return Some(0);
    }

    let mut found = 0;
    for pos in (0..count).map(|i| ids + i * width) {
        if read_uint(reference, pos, width) == Some(0) {
            found += 1;
            if patch {
                reference[pos..pos + width].copy_from_slice(&primary.to_be_bytes()[4 - width..]);
            }
        }
    }
    Some(found)
}

/// Replaces the 'tmap' compatible brand with the major brand, so libavif skips the gain map and
/// shows the base image
pub fn hide_gain_map(data: &mut [u8]) -> bool {
    let Some((kind, ftyp)) = children(data, 0..data.len()).next() else {
        return false;
    };
    if kind != *b"ftyp" || ftyp.len() < 8 {
        return false;
    }

    let major: [u8; 4] = data[ftyp.start..ftyp.start + 4].try_into().unwrap();
    if major == *b"tmap" {
        return false;
    }

    let mut changed = false;
    for brand in data[ftyp.start + 8..ftyp.end].as_chunks_mut::<4>().0 {
        if brand == b"tmap" {
            *brand = major;
            changed = true;
        }
    }
    changed
}

pub fn split_duplicate_ipma(data: &mut [u8]) -> bool {
    let Some(meta) = find(data, 0..data.len(), b"meta") else {
        return false;
    };

    if meta.len() < 4 {
        return false;
    }

    let Some(iprp) = find(data, meta.start + 4..meta.end, b"iprp") else {
        return false;
    };

    let ipma: Vec<usize> = children(data, iprp)
        .filter(|(kind, body)| kind == b"ipma" && body.len() >= 4)
        .map(|(_, body)| body.start)
        .take(MAX_IPMA + 1)
        .collect();

    if ipma.len() > MAX_IPMA {
        return false;
    }

    let tags: Vec<u32> = ipma
        .iter()
        .map(|&start| u32::from_be_bytes(data[start..start + 4].try_into().unwrap()))
        .collect();

    if tags.iter().any(|&tag| tag & !KNOWN_IPMA_BITS != 0) {
        return false;
    }

    let mut seen = Vec::new();
    let mut changed = false;
    for (start, mut tag) in ipma.into_iter().zip(tags) {
        if seen.contains(&tag) {
            let Some(unique) = (1..24)
                .map(|bit| tag | (1 << bit))
                .find(|t| !seen.contains(t))
            else {
                return changed;
            };
            tag = unique;
            data[start..start + 4].copy_from_slice(&tag.to_be_bytes());
            changed = true;
        }
        seen.push(tag);
    }
    changed
}
