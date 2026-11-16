//! Reading the drawing's geographic location (GEODATA, model space) to pull
//! its EPSG code. Open CAD Studio v2026.46 writes it with `GEOGRAPHICLOCATION`
//! (or `HostApi::set_geo_location`) as MapGuide XML with an EPSG alias, WKT,
//! or `EPSG:<code>`.

use ocs_plugin_api::host::codec::objects::ObjectType;
use ocs_plugin_api::host::Document;

/// EPSG code found in one coordinate-system definition text, if any.
pub fn epsg_from_definition(text: &str) -> Option<u16> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    if t.starts_with('<') {
        return xml_alias_code(t);
    }
    if t.contains('[') {
        return wkt_root_code(t);
    }
    let up = t.to_ascii_uppercase();
    up.strip_prefix("EPSG:").unwrap_or(&up).trim().parse().ok()
}

/// `<Alias id="32610" type="CoordinateSystem">` of a MapGuide definition.
fn xml_alias_code(xml: &str) -> Option<u16> {
    let mut rest = xml;
    while let Some(i) = rest.find("<Alias") {
        let tag_end = rest[i..].find('>').map(|e| i + e)?;
        let tag = &rest[i..tag_end];
        if tag.contains("CoordinateSystem") {
            let id = tag.split("id=\"").nth(1)?.split('"').next()?;
            if let Ok(c) = id.trim().parse() {
                return Some(c);
            }
        }
        rest = &rest[tag_end..];
    }
    None
}

/// EPSG AUTHORITY / ID sitting directly on the root WKT node (a nested GEOGCS
/// authority such as 4326 is not the CRS of the drawing).
fn wkt_root_code(wkt: &str) -> Option<u16> {
    let s: String = wkt.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_uppercase();
    let b = s.as_bytes();
    let (mut depth, mut quoted, mut found) = (0usize, false, None);
    let mut i = s.find('[')?;
    while i < b.len() {
        match b[i] {
            b'"' => quoted = !quoted,
            b'[' if !quoted => depth += 1,
            b']' if !quoted => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return found;
                }
            }
            _ if quoted || depth != 1 => {}
            _ => {
                for p in ["AUTHORITY[\"EPSG\",\"", "ID[\"EPSG\","] {
                    if s[i..].starts_with(p) {
                        let d: String = s[i + p.len()..].chars().take_while(|c| c.is_ascii_digit()).collect();
                        found = d.parse().ok().or(found);
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    found
}

/// EPSG code of the drawing's model-space geographic location, if one is set.
pub fn drawing_epsg(doc: &Document) -> Option<u16> {
    let ms = doc.block_records.get("*Model_Space")?.handle;
    let dict = doc.extension_dictionary_handle(ms)?;
    let ObjectType::Dictionary(d) = doc.objects.get(&dict)? else { return None };
    for (key, h) in d.entries.iter() {
        if !key.eq_ignore_ascii_case("ACAD_GEOGRAPHICDATA") {
            continue;
        }
        if let Some(ObjectType::GeoData(g)) = doc.objects.get(h) {
            return epsg_from_definition(&g.coordinate_system_definition)
                .or_else(|| epsg_from_definition(&g.coordinate_system_wkt));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_and_prefixed() {
        assert_eq!(epsg_from_definition("EPSG:32610"), Some(32610));
        assert_eq!(epsg_from_definition(" epsg:2193 "), Some(2193));
        assert_eq!(epsg_from_definition("2193"), Some(2193));
        assert_eq!(epsg_from_definition(""), None);
        assert_eq!(epsg_from_definition("+proj=utm +zone=10"), None);
    }

    #[test]
    fn mapguide_xml() {
        let x = "<?xml version=\"1.0\"?>\n<Dictionary>\n<Alias id=\"32610\" type=\"CoordinateSystem\">\n<ObjectId>UTM84-10N</ObjectId>\n</Alias>\n</Dictionary>";
        assert_eq!(epsg_from_definition(x), Some(32610));
        assert_eq!(epsg_from_definition("<a><Alias id=\"x\" type=\"Other\"></Alias></a>"), None);
    }

    #[test]
    fn wkt_root_only() {
        let w = "PROJCS[\"WGS 84 / UTM zone 10N\",GEOGCS[\"WGS 84\",AUTHORITY[\"EPSG\",\"4326\"]],PROJECTION[\"Transverse_Mercator\"],AUTHORITY[\"EPSG\",\"32610\"]]";
        assert_eq!(epsg_from_definition(w), Some(32610));
        let custom = "PROJCS[\"Custom TM\",GEOGCS[\"WGS 84\",AUTHORITY[\"EPSG\",\"4326\"]],PROJECTION[\"Transverse_Mercator\"]]";
        assert_eq!(epsg_from_definition(custom), None);
        let v2 = "PROJCRS[\"NZGD2000 / NZTM\",BASEGEOGCRS[\"NZGD2000\",ID[\"EPSG\",4167]],ID[\"EPSG\",2193]]";
        assert_eq!(epsg_from_definition(v2), Some(2193));
    }
}
