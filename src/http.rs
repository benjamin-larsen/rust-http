#[derive(Debug)]
pub(crate) struct RequestLine {
    pub method: String,
    pub path: String,
    pub version: String,
}

pub(crate) fn decode_request_line(line: String) -> Option<RequestLine> {
    let mut split = line.splitn(3, ' ');

    let Some(method) = split.next() else { return None };
    let Some(path) = split.next() else { return None };
    let Some(version) = split.next() else { return None };

    return Some(RequestLine {
        method: String::from(method),
        path: String::from(path),
        version: String::from(version),
    });
}

pub(crate) fn decode_header(line: String) -> Option<(String, String)> {
    let mut split = line.splitn(2, ':');

    let Some(key) = split.next() else { return None };
    let Some(value) = split.next() else { return None };

    return Some((
        String::from(key),
        String::from(value.trim_matches([' ', '\t']))
    ));
}