use il;
use clang as cx;

static PREFIX: &'static str = "POSGEN_VAR_";

struct Context<'a> {
    file: &'a [u8],
    defs: &'a ::Defs,
    globals: Vec<il::Global>,
}

impl<'a> Context<'a> {
    fn conv_ptr_ty(&mut self, ty: &cx::Type) -> String {
        let mutable = if ty.is_const() {
            "const"
        } else {
            "mut"
        };
        match ty.kind() {
            cx::ll::CXType_Unexposed | cx::ll::CXType_FunctionProto
                    | cx::ll::CXType_FunctionNoProto => {
                let ret_ty = ty.ret_type();
                let decl = ty.declaration();
                if ret_ty.kind() != cx::ll::CXType_Invalid {
                    let mut ret = String::new();
                    ret.push_str("fn(");
                    for arg in ty.arg_types().iter() {
                        ret.push_str(self.type_to_str(arg).as_slice());
                        ret.push_str(", ");
                    }
                    ret.push_str(")");
                    match ret_ty.kind() {
                        cx::ll::CXType_Void => { },
                        _ => ret.push_str(format!(" -> {}", self.type_to_str(&ret_ty)).as_slice()),
                    }
                    ret
                } else if decl.kind() != cx::ll::CXCursor_NoDeclFound {
                    format!("*{} {}", mutable, self.type_to_str(&decl.cur_type()))
                } else {
                    format!("*{} ::void_t", mutable)
                }
            }
            _ => {
                format!("*{} {}", mutable, self.type_to_str(ty))
            }
        }
    }

    fn type_to_str(&mut self, ty: &cx::Type) -> String {
        self.type_to_str_int(ty, true)
    }

    fn find_type(&self, spelling: &str, skip: bool) -> Option<String> {
        let trimmed = spelling.as_slice().trim_left_chars('_');
        match self.defs.types.iter().find(|t| t.as_slice() == trimmed) {
            Some(s) => if skip { return Some(s.to_string()) },
            _ => match ::types::find(trimmed) {
                Some(t) => return Some(t.to_string()),
                _ => { },
            },
        }
        None
    }

    fn type_to_str_int(&mut self, ty: &cx::Type, skip: bool) -> String {
        match ty.kind() {
            cx::ll::CXType_SChar | cx::ll::CXType_Char_S => "::schar_t".to_string(),
            cx::ll::CXType_UChar | cx::ll::CXType_Char_U => "::uchar_t".to_string(),
            cx::ll::CXType_Void       => "::void_t".to_string(),
            cx::ll::CXType_Bool       => "::bool_t".to_string(),
            cx::ll::CXType_UShort     => "::ushort_t".to_string(),
            cx::ll::CXType_UInt       => "::uint_t".to_string(),
            cx::ll::CXType_ULong      => "::ulong_t".to_string(),
            cx::ll::CXType_ULongLong  => "::ulonglong_t".to_string(),
            cx::ll::CXType_Short      => "::short_t".to_string(),
            cx::ll::CXType_Int        => "::int_t".to_string(),
            cx::ll::CXType_Long       => "::long_t".to_string(),
            cx::ll::CXType_LongLong   => "::longlong_t".to_string(),
            cx::ll::CXType_Float      => "::float_t".to_string(),
            cx::ll::CXType_Double     => "::double_t".to_string(),
            cx::ll::CXType_LongDouble => "::longdouble_t".to_string(),
            cx::ll::CXType_Pointer => self.conv_ptr_ty(&ty.pointee_type()),
            cx::ll::CXType_Record | cx::ll::CXType_Unexposed | cx::ll::CXType_Typedef => {
                let spelling = ty.spelling();
                match self.find_type(spelling.as_slice(), skip) {
                    Some(s) => return s,
                    _ => { }
                }
                let cursor = ty.declaration();
                match cursor.kind() {
                    cx::ll::CXCursor_StructDecl | cx::ll::CXCursor_UnionDecl => {
                        let spelling = cursor.spelling();
                        match self.find_type(spelling.as_slice(), skip) {
                            Some(s) => s,
                            _ => unknown_array(ty),
                        }
                    },
                    cx::ll::CXCursor_EnumDecl => {
                        match ty.size() {
                            1 => "i8".to_string(),
                            2 => "i16".to_string(),
                            4 => "i32".to_string(),
                            8 => "i64".to_string(),
                            _ => panic!("Enum type too large"),
                        }
                    },
                    cx::ll::CXCursor_TypedefDecl => {
                        let ty = cursor.typedef_type();
                        self.type_to_str_int(&ty, skip)
                    },
                    _ => format!("UNKNOWN: {} {}", cursor.kind(), ty.kind()),
                }
            },
            cx::ll::CXType_Enum => {
                match ty.size() {
                    1 => "i8".to_string(),
                    2 => "i16".to_string(),
                    4 => "i32".to_string(),
                    8 => "i64".to_string(),
                    _ => panic!("Enum type too large"),
                }
            },
            cx::ll::CXType_ConstantArray => {
                let ts = self.type_to_str(&ty.elem_type());
                format!("[{}, ..{}]", ts, ty.array_size())
            }
            _ => format!("Can't handle type {}", ty.kind()),
        }
    }

    fn visit_struct(&mut self, cursor: &cx::Cursor,
                    fields: &mut Vec<il::Field>) -> cx::ll::Enum_CXVisitorResult {
        if cursor.kind() == cx::ll::CXCursor_FieldDecl {
            let ty = cursor.cur_type();
            let name = cursor.spelling();
            let public = !name.as_slice().starts_with("_");
            let ts = if public {
                self.type_to_str(&ty)
            } else {
                unknown_array(&ty)
            };
            let field = il::Field { name: name, ty: ts, public: public };
            fields.push(field);
        }
        cx::ll::CXChildVisit_Continue
    }

    fn visit_var(&mut self, cursor: &cx::Cursor,
                 dst: &mut Option<String>) -> cx::ll::Enum_CXVisitorResult {
        match cursor.kind() {
            cx::ll::CXCursor_DeclRefExpr => {
                let def = cursor.definition();
                if def.kind() == cx::ll::CXCursor_EnumConstantDecl {
                    *dst = Some(format!("{}", def.enum_val()));
                }
                cx::ll::CXChildVisit_Break
            },
            cx::ll::CXCursor_ParenExpr | cx::ll::CXCursor_IntegerLiteral
                    | cx::ll::CXCursor_FloatingLiteral => {
                let offset = cursor.get_location();
                let mut slc = self.file.slice_from(offset);
                slc = match slc.iter().position(|b| *b == ';' as u8) {
                    Some(p) => slc.slice_to(p),
                    _ => slc,
                };
                *dst = ::std::str::from_utf8(slc).map(|s| s.to_string());
                cx::ll::CXChildVisit_Break
            },
            _ => cx::ll::CXChildVisit_Recurse,
        }
    }

    fn handle_struct(&mut self, cursor: &cx::Cursor,
                     spelling: String) -> cx::ll::Enum_CXVisitorResult {
        let mut fields = vec!();
        cursor.visit(|c, _| {
            self.visit_struct(c, &mut fields)
        });
        let s = il::Struct {
            name: spelling,
            fields: fields,
        };
        self.globals.push(il::Global::StructVar(s));
        if cursor.kind() == cx::ll::CXCursor_FieldDecl {
            cx::ll::CXChildVisit_Break
        } else {
            cx::ll::CXChildVisit_Recurse
        }
    }

    fn handle_typedef(&mut self, cursor: &cx::Cursor,
                      spelling: String) -> cx::ll::Enum_CXVisitorResult {
        let mut dst = cursor.typedef_type();
        if dst.kind() == cx::ll::CXType_Unexposed {
            dst = dst.canonical_type();
        }
        if dst.kind() == cx::ll::CXType_Record {
            let decl = dst.declaration();
            self.handle_struct(&decl, spelling);
        } else {
            let dst = self.type_to_str_int(&dst, false);
            let typedef = il::Typedef { name: spelling, dst: dst };
            self.globals.push(il::Global::TypedefVar(typedef));
        }
        cx::ll::CXChildVisit_Continue
    }

    fn visit_top(&mut self, cursor: &cx::Cursor) -> cx::ll::Enum_CXVisitorResult {
        let kind = cursor.kind();
        let spelling = cursor.spelling();
        match kind {
            cx::ll::CXCursor_StructDecl | cx::ll::CXCursor_TypedefDecl => {
                if self.defs.types.iter().all(|t| t.as_slice() != spelling.as_slice()) {
                    return cx::ll::CXChildVisit_Continue;
                }
            },
            cx::ll::CXCursor_VarDecl => {
                if !spelling.as_slice().starts_with(PREFIX) {
                    return cx::ll::CXChildVisit_Continue;
                }
            },
            _ => { },
        }
        match kind {
            cx::ll::CXCursor_StructDecl => self.handle_struct(cursor, spelling),
            cx::ll::CXCursor_TypedefDecl => self.handle_typedef(cursor, spelling),
            cx::ll::CXCursor_VarDecl => {
                let real_name = spelling.as_slice().slice_from(PREFIX.len());
                let mut val = None;
                cursor.visit(|c, _| {
                    self.visit_var(c, &mut val)
                });
                let var = self.defs.consts.iter().find(|v| v.name.as_slice() == real_name);
                match (var, val) {
                    (Some(var), Some(mut val)) => {
                        if val.len() > 1 && val.as_bytes()[0] == '0' as u8 &&
                                val.as_bytes()[1] != 'x' as u8 {
                            val = format!("0o{}", val.as_slice().slice_from(1));
                        }
                        let info = il::Constant {
                            name: real_name.to_string(),
                            ty: var.rs_type.clone(),
                            val: val
                        };
                        self.globals.push(il::Global::ConstantVar(info));
                    },
                    _ => { },
                }
                cx::ll::CXChildVisit_Continue
            }
            cx::ll::CXCursor_EnumDecl     => cx::ll::CXChildVisit_Continue,
            cx::ll::CXCursor_FunctionDecl => cx::ll::CXChildVisit_Continue,
            cx::ll::CXCursor_FieldDecl    => cx::ll::CXChildVisit_Continue,
            _ => cx::ll::CXChildVisit_Recurse,
        }
    }
}

fn unknown_array(ty: &cx::Type) -> String {
    let size = ty.size();
    let mut align = ty.align();
    if align == 0 {
        align = 1;
    }
    assert_eq!(size % align, 0);
    let ut = match align {
        0 | 1 => "u8",
        2 => "u16",
        4 => "u32",
        8 => "u64",
        _ => unreachable!(),
    };
    format!("[{}, ..{}]", ut, size / align)
}

fn preprocess(defs: &::Defs) -> ::std::io::IoResult<Vec<u8>> {
    let mut wr = ::std::io::MemWriter::new();
    try!(writeln!(&mut wr, "#include <{}>", defs.header.as_slice()));
    for c in defs.consts.iter() {
        try!(writeln!(&mut wr, "static const {} {}{} = {};", c.c_type.as_slice(), PREFIX,
                      c.name.as_slice(), c.name.as_slice()));
    }
    let header = wr.unwrap();
    let mut process = try!(::std::io::Command::new("clang").args(&["-P", "-E", "-"]).spawn());
    process.stdin.as_mut().unwrap().write(header.as_slice()).ok();
    let output = try!(process.wait_with_output());
    if !output.status.success() {
        let mut err = ::std::io::standard_error(::std::io::InvalidInput);
        err.detail = Some(String::from_utf8_lossy(output.error.as_slice()).into_string());
        return Err(err);
    }
    Ok(output.output)
}

pub fn parse(defs: &::Defs) -> ::std::io::IoResult<Vec<il::Global>> {
    let pp = try!(preprocess(defs));
    let tmpdir = match ::std::io::TempDir::new("") {
        Ok(d) => d,
        _ => return Err(::std::io::IoError::last_error())
    };
    let mut path = tmpdir.path().clone();
    path.push("file.c");
    let mut file = try!(::std::io::fs::File::create(&path));
    try!(file.write(pp.as_slice()));
    drop(file);

    let mut ctx = Context {
        file: pp.as_slice(),
        defs: defs,
        globals: vec!(),
    };

    let ix = cx::Index::create(false, true);
    if ix.is_null() {
        return Err(::std::io::standard_error(::std::io::OtherIoError))
    }

    let unit = cx::TranslationUnit::parse(&ix, &path);
    if unit.is_null() {
        return Err(::std::io::standard_error(::std::io::OtherIoError))
    }

    let cursor = unit.cursor();

    cursor.visit(|cur, _| ctx.visit_top(cur));

    unit.dispose();
    ix.dispose();

    Ok(ctx.globals)
}
