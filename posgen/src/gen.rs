use il;

pub fn gen(globals: Vec<il::Global>) {
    for g in globals.iter() {
        match *g {
            il::Typedef(ref v) => {
                println!("pub type {} = {};", v.name, v.dst);
            },
            il::Struct(ref v) => {
                println!("#[repr(C)]");
                println!("pub struct {} {{", v.name);
                if v.fields.len() == 0 {
                    println!("    _dummy: (),");
                }
                for f in v.fields.iter() {
                    print!("    ");
                    if f.public {
                        print!("pub ");
                    }
                    println!("{}: {},", f.name, f.ty);
                }
                println!("}}");
                println!("new!({})", v.name);
            },
            il::Constant(ref v) => {
                println!("pub static {}: {} = {};", v.name, v.ty, v.val);
            },
        }
    }
}
