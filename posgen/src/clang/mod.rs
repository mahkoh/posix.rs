use libc::{c_uint, c_int};
use std::{ffi, str, mem, ptr};
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher, Writer};

pub mod ll;

// Cursor
pub struct Cursor {
    x: ll::CXCursor
}

pub type CursorVisitor<'s> = &'s mut(FnMut(&Cursor, &Cursor) -> ll::Enum_CXChildVisitResult + 's);

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

    pub fn visit(&self, mut func: CursorVisitor) {
        unsafe {
            let data = mem::transmute::<&mut CursorVisitor, ll::CXClientData>(&mut func);
            let f: extern fn(cur: ll::CXCursor, parent: ll::CXCursor,
                         data: ll::CXClientData) -> ll::Enum_CXChildVisitResult = visit_children;
            ll::clang_visitChildren(self.x, Some(f), data);
        };
    }

    pub fn enum_val(&self) -> i64 {
        unsafe { ll::clang_getEnumConstantDeclValue(self.x) as i64 }
    }

    // typedef
    pub fn typedef_type(&self) -> Type {
        unsafe { Type { x: ll::clang_getTypedefDeclUnderlyingType(self.x) } }
    }

    pub fn get_location(&self) -> usize {
        unsafe {
            let loc = ll::clang_getCursorLocation(self.x);
            let mut offset = 0;
            ll::clang_getSpellingLocation(loc, 0 as *mut _, 0 as *mut _, 0 as *mut _,
                                          &mut offset as *mut _);
            offset as usize
        }
    }
}

extern fn visit_children(cur: ll::CXCursor, parent: ll::CXCursor,
                         data: ll::CXClientData) -> ll::Enum_CXChildVisitResult {
    unsafe {
        let mut func = mem::transmute::<ll::CXClientData, &mut CursorVisitor>(data);
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

impl<H: Hasher+Writer> Hash<H> for Cursor {
    fn hash(&self, state: &mut H) {
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

    pub fn size(&self) -> usize {
        unsafe {
            let val = ll::clang_Type_getSizeOf(self.x);
            if val < 0 { 0 } else { val as usize }
        }
    }

    pub fn align(&self) -> usize {
        unsafe {
            let val = ll::clang_Type_getAlignOf(self.x);
            if val < 0 { 0 } else { val as usize }
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

    pub fn array_size(&self) -> usize {
        unsafe {
            ll::clang_getArraySize(self.x) as usize
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
    pub fn location(&self) -> (File, usize, usize, usize) {
        unsafe {
            let mut file = ptr::null_mut();
            let mut line = 0;
            let mut col = 0;
            let mut off = 0;
            ll::clang_getSpellingLocation(self.x, &mut file, &mut line, &mut col, &mut off);
            return (File { x: file }, line as usize, col as usize, off as usize);
        }
    }
}

impl fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (file, line, col, _) = self.location();
        match file.is_null() {
            false => {
                try!(write!(f, "{}", file.name()));
                try!(write!(f, ":"));
                try!(write!(f, "{}", line));
                try!(write!(f, ":"));
                write!(f, "{}", col)
            },
            true => write!(f, "builtin definitions"),
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

impl fmt::Display for String_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x.data.is_null() {
            return "".fmt(f);
        }
        unsafe {
            let c_str = ll::clang_getCString(self.x);
            str::from_utf8(ffi::c_str_to_bytes(&c_str)).unwrap().fmt(f)
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
        let _fname = ffi::CString::from_slice(file.as_vec());
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
