use std::fs::File;

use anyhow::Result;

use uesave::{Property, Save, StructValue, ValueArray};

#[rustfmt::skip]
fn main() -> Result<()> {
    let save = Save::read(&mut File::open(
        "examples/space-rig-decorator/PropPack.sav",
    )?)?;
    let props = &save.root.properties.get(0, "PropList").unwrap();
    if let Property::Array { value: ValueArray::Struct { value, .. }, .. } = props {
        for prop in value {
            if let StructValue::Struct(p) = prop {
                if let Property::Struct { value: StructValue::Struct(value), .. } = p.get(0, "PropPosition_7_B8CD81CD4E138D8E06FBBA8056FE4C85").unwrap() {
                    if let Property::Struct { value: StructValue::Quat(value) , .. } = value.get(0, "Rotation").unwrap() {
                        print!("{}:{}:{}:{}:", value.x, value.y, value.z, value.w);
                    }
                    if let Property::Struct { value: StructValue::Vector(value), .. } = value.get(0, "Translation").unwrap() {
                        print!("{}:{}:{}:", value.x, value.y, value.z);
                    }
                    if let Property::Struct { value: StructValue::Vector(value), .. } = value.get(0, "Scale3D").unwrap() {
                        print!("{}:{}:{}:", value.x, value.y, value.z);
                    }
                }
                if let Property::Str { value, .. } = p.get(0, "PropName_10_4BB2A20D47DA97D8ECB7D888147BBB97").unwrap() {
                    print!("{value}:");
                }
                if let Property::Bool { value, .. } = p.get(0, "IsStaticMesh_20_AB977B2F47FD519F53FB8CB85490631B").unwrap() {
                    print!("{value}:");
                }
                if let Property::Object { value, .. } = p.get(0, "DynamicPropClass_19_AA6C35BE4D24AB6B42E8999E55661065").unwrap() {
                    print!("{value}:");
                }
                if let Property::Object { value, .. } = p.get(0, "StaticMesh_18_BAF2BF524DA3EA4A985B9D87B727223A").unwrap() {
                    println!("{value}");
                }
            }
        }
    }
    Ok(())
}
