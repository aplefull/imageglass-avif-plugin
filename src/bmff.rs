use std::ops::Range;

const MAX_IPMA: usize = 4;
const KNOWN_IPMA_BITS: u32 = 0x0100_0001;

fn children(data: &[u8], range: Range<usize>) -> impl Iterator<Item = ([u8; 4], Range<usize>)> + '_ {
    let mut pos = range.start;
    std::iter::from_fn(move || {
        let header = data.get(pos..(pos + 8).min(range.end))?.get(..8)?;
        let kind: [u8; 4] = header[4..].try_into().unwrap();
        let mut body = pos + 8;

        let end = match u32::from_be_bytes(header[..4].try_into().unwrap()) {
            0 => range.end,
            1 => {
                let large = data.get(body..(body + 8).min(range.end))?.get(..8)?;
                body += 8;
                let size = usize::try_from(u64::from_be_bytes(large.try_into().unwrap())).ok()?;
                pos.checked_add(size)?
            }
            size => pos.checked_add(size as usize)?,
        };
        if end > range.end || end < body {
            return None;
        }

        pos = end;
        Some((kind, body..end))
    })
}

fn find(data: &[u8], range: Range<usize>, kind: &[u8; 4]) -> Option<Range<usize>> {
    children(data, range)
        .find(|(k, _)| k == kind)
        .map(|(_, body)| body)
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
            let Some(unique) = (1..24).map(|bit| tag | (1 << bit)).find(|t| !seen.contains(t)) else {
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
