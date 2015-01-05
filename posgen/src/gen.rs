use il;

pub fn gen(globals: Vec<il::Global>) {
    for g in globals.iter() {
        match *g {
            il::Global::TypedefVar(ref v) => {
                println!("pub type {} = {};", v.name, v.dst);
            },
            il::Global::StructVar(ref v) => {
                println!("#[repr(C)]");
                println!("#[derive(Copy)]");
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
                println!("new!({});", v.name);
            },
            il::Global::ConstantVar(ref v) => {
                println!("pub const {}: {} = {};", v.name, v.ty, v.val);
            },
        }
    }
}
