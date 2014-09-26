use libc::{c_uint, c_int};
use std::{mem, ptr, string};
use std::fmt;
use std::hash::Hash;
use std::hash::sip::SipState;

pub mod ll;

// Cursor
pub struct Cursor {
    x: ll::CXCursor
}

pub type CursorVisitor<'s> = |c: &Cursor, p: &Cursor|: 's -> ll::Enum_CXChildVisitResult;

impl Cursor {
    // common
    pub fn spelling(&self) -> String {
        unsafe { String_ { x: ll::clang_getCursorSpelling(self.x) }.to_string() }
    }

    pub fn kind(&self) -> ll::Enum_CXCursorKind {
        unsafe { ll::clang_getCursorKind(self.x) }
    }

    pub fn cur_type(&self) -> Type {
        unsafe { Type { x: ll::clang_getCursorType(self.x) } }
    }

    pub fn definition(&self) -> Cursor {
        unsafe { Cursor { x: ll::clang_getCursorDefinition(self.x) } }
    }

    pub fn visit(&self, func: CursorVisitor) {
        unsafe {
            let data = mem::transmute::<&CursorVisitor, ll::CXClientData>(&func);
            ll::clang_visitChildren(self.x, Some(visit_children), data);
        };
    }

    pub fn enum_val(&self) -> i64 {
        unsafe { ll::clang_getEnumConstantDeclValue(self.x) as i64 }
    }

    // typedef
    pub fn typedef_type(&self) -> Type {
        unsafe { Type { x: ll::clang_getTypedefDeclUnderlyingType(self.x) } }
    }

    pub fn get_location(&self) -> uint {
        unsafe {
            let loc = ll::clang_getCursorLocation(self.x);
            let mut offset = 0;
            ll::clang_getSpellingLocation(loc, 0 as *mut _, 0 as *mut _, 0 as *mut _,
                                          &mut offset as *mut _);
            offset as uint
        }
    }
}

extern fn visit_children(cur: ll::CXCursor, parent: ll::CXCursor,
                         data: ll::CXClientData) -> ll::Enum_CXChildVisitResult {
    unsafe {
        let func = mem::transmute::<ll::CXClientData, &mut CursorVisitor>(data);
        return (*func)(&Cursor { x: cur }, &Cursor { x: parent });
    }
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Cursor) -> bool {
        unsafe { ll::clang_equalCursors(self.x, other.x) == 1 }
    }

    fn ne(&self, other: &Cursor) -> bool {
        return !self.eq(other);
    }
}

impl Eq for Cursor {}

impl Hash for Cursor {
    fn hash(&self, state: &mut SipState) {
        self.x.kind.hash(state);
        self.x.xdata.hash(state);
        self.x.data[0].hash(state);
        self.x.data[1].hash(state);
        self.x.data[2].hash(state);
    }
}

// type
pub struct Type {
    x: ll::CXType
}

impl Type {
    // common
    pub fn kind(&self) -> ll::Enum_CXTypeKind {
        return self.x.kind;
    }

    pub fn spelling(&self) -> String {
        unsafe { String_ { x: ll::clang_getTypeSpelling(self.x) }.to_string() }
    }

    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor { x: ll::clang_getTypeDeclaration(self.x) }
        }
    }

    pub fn is_const(&self) -> bool {
        unsafe {
            ll::clang_isConstQualifiedType(self.x) == 1
        }
    }

    pub fn size(&self) -> uint {
        unsafe {
            let val = ll::clang_Type_getSizeOf(self.x);
            if val < 0 { 0 } else { val as uint }
        }
    }

    pub fn align(&self) -> uint {
        unsafe {
            let val = ll::clang_Type_getAlignOf(self.x);
            if val < 0 { 0 } else { val as uint }
        }
    }

    // pointer
    pub fn pointee_type(&self) -> Type {
        unsafe {
            Type { x: ll::clang_getPointeeType(self.x) }
        }
    }

    // array
    pub fn elem_type(&self) -> Type {
        unsafe {
            Type { x: ll::clang_getArrayElementType(self.x) }
        }
    }

    pub fn array_size(&self) -> uint {
        unsafe {
            ll::clang_getArraySize(self.x) as uint
        }
    }

    // typedef
    pub fn canonical_type(&self) -> Type {
        unsafe {
            Type { x: ll::clang_getCanonicalType(self.x) }
        }
    }

    pub fn arg_types(&self) -> Vec<Type> {
        unsafe {
            let num = ll::clang_getNumArgTypes(self.x);
            let mut args = vec!();
            for i in range(0, num) {
                args.push(Type { x: ll::clang_getArgType(self.x, i as c_uint) });
            }
            return args;
        }
    }

    pub fn ret_type(&self) -> Type {
        unsafe {
            Type { x: ll::clang_getResultType(self.x) }
        }
    }
}

// SourceLocation
pub struct SourceLocation {
    x: ll::CXSourceLocation
}

impl SourceLocation {
    pub fn location(&self) -> (File, uint, uint, uint) {
        unsafe {
            let mut file = ptr::null_mut();
            let mut line = 0;
            let mut col = 0;
            let mut off = 0;
            ll::clang_getSpellingLocation(self.x, &mut file, &mut line, &mut col, &mut off);
            return (File { x: file }, line as uint, col as uint, off as uint);
        }
    }
}

impl fmt::Show for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (file, line, col, _) = self.location();
        match file.is_null() {
            false => {
                try!(file.name().fmt(f));
                try!(":".fmt(f));
                try!(line.fmt(f));
                try!(":".fmt(f));
                col.fmt(f)
            },
            true => "builtin definitions".fmt(f)
        }
    }
}

// File
pub struct File {
    x: ll::CXFile
}

impl File {
    pub fn name(&self) -> String {
        if self.is_null() {
            return "".to_string();
        }
        unsafe {
            String_ { x: ll::clang_getFileName(self.x) }.to_string()
        }
    }

    pub fn is_null(&self) -> bool {
        self.x.is_null()
    }
}

// String
pub struct String_ {
    x: ll::CXString
}

impl fmt::Show for String_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x.data.is_null() {
            return "".fmt(f);
        }
        unsafe {
            let c_str = ll::clang_getCString(self.x) as *const u8;
            string::raw::from_buf(c_str).fmt(f)
        }
    }
}

// Index
pub struct Index {
    x: ll::CXIndex
}

impl Index {
    pub fn create(pch: bool, diag: bool) -> Index {
        unsafe {
            Index { x: ll::clang_createIndex(pch as c_int, diag as c_int) }
        }
    }

    pub fn dispose(&self) {
        unsafe {
            ll::clang_disposeIndex(self.x);
        }
    }

    pub fn is_null(&self) -> bool {
        self.x.is_null()
    }
}

// TranslationUnit
pub struct TranslationUnit {
    x: ll::CXTranslationUnit
}

impl TranslationUnit {
    pub fn parse(ix: &Index, file: &Path) -> TranslationUnit {
        let _fname = file.to_c_str();
        let fname = _fname.as_ptr();
        let c_args = vec!();
        let mut c_unsaved = vec!();
        let tu = unsafe {
            ll::clang_parseTranslationUnit(ix.x, fname,
                                       c_args.as_ptr(),
                                       c_args.len() as c_int,
                                       c_unsaved.as_mut_ptr(),
                                       c_unsaved.len() as c_uint,
                                       0)
        };
        TranslationUnit { x: tu }
    }

    pub fn cursor(&self) -> Cursor {
        unsafe { Cursor { x: ll::clang_getTranslationUnitCursor(self.x) } }
    }

    pub fn dispose(&self) {
        unsafe { ll::clang_disposeTranslationUnit(self.x); }
    }

    pub fn is_null(&self) -> bool {
        self.x.is_null()
    }
}
