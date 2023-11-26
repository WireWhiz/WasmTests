#![feature(raw_ref_op)]
#[derive(Clone)]
#[repr(C)]
pub struct TestComponent {
    pub a: bool,
    pub b: i32,
    pub c: f32
}

#[no_mangle]
pub extern "C" fn test_component_access(mut component: Box<TestComponent>) -> Box<TestComponent> {
    *component = TestComponent {
        a: true,
        b: 42,
        c: 3.14
    };
    component
}



// ----------------- Expanded macros from brane_engine_api_macros -----------------
#[repr(align(4))]
#[repr(C)]
pub struct ComponentFieldInfo {
    pub offset: u32,
    pub size: u32,
    pub name: &'static str,
    pub ty: &'static str,

}

#[repr(align(4))]
#[repr(C)]
pub struct ComponentInfo {
    pub size: u32,
    pub fields: &'static [ComponentFieldInfo]
}

#[allow(non_upper_case_globals)]
static be_info_data_TestComponent: ComponentInfo =
    ComponentInfo {
        size: std::mem::size_of::<TestComponent>() as u32,
        fields: &[ComponentFieldInfo {
            offset:
            {
                let uninit =
                    memoffset::__priv::mem::MaybeUninit::<TestComponent>::uninit();
                let base_ptr: *const TestComponent = uninit.as_ptr();
                let field_ptr =
                    {
                        #[allow(clippy :: unneeded_field_pattern)]
                            let TestComponent { a: _, .. };
                        let base = base_ptr;

                        #[allow(unused_unsafe)]
                        unsafe {
                            { &raw const (*(base as *const TestComponent)).a }
                        }
                    };
                {
                    let field = field_ptr;
                    let base = base_ptr;
                    unsafe {
                        (field as *const u8).offset_from(base as *const u8) as usize
                    }
                }
            } as u32,
            size: std::mem::size_of::<bool>() as u32,
            name: "a",
            ty: "bool",
        },
            ComponentFieldInfo {
                offset: {
                    let uninit =
                        ::memoffset::__priv::mem::MaybeUninit::<TestComponent>::uninit();
                    let base_ptr: *const TestComponent = uninit.as_ptr();
                    let field_ptr =
                        {
                            #[allow(clippy :: unneeded_field_pattern)]
                                let TestComponent { b: _, .. };
                            let base = base_ptr;

                            #[allow(unused_unsafe)]
                            unsafe {
                                { &raw const (*(base as *const TestComponent)).b }
                            }
                        };
                    {
                        let field = field_ptr;
                        let base = base_ptr;
                        unsafe {
                            (field as *const u8).offset_from(base as *const u8) as usize
                        }
                    }
                } as u32,
                size: std::mem::size_of::<i32>() as u32,
                name: "b",
                ty: "i32",
            },
            ComponentFieldInfo {
                offset: {
                    let uninit = memoffset::__priv::mem::MaybeUninit::<TestComponent>::uninit();
                    let base_ptr: *const TestComponent = uninit.as_ptr();
                    let field_ptr =
                        {
                            #[allow(clippy :: unneeded_field_pattern)]
                                let TestComponent { c: _, .. };
                            let base = base_ptr;

                            #[allow(unused_unsafe)]
                            unsafe {
                                { &raw const (*(base as *const TestComponent)).c }
                            }
                        };
                    {
                        let field = field_ptr;
                        let base = base_ptr;
                        unsafe {
                            (field as *const u8).offset_from(base as *const u8) as usize
                        }
                    }
                } as u32,
                size: std::mem::size_of::<f32>() as u32,
                name: "c",
                ty: "f32",
            }],
    };

#[no_mangle]
pub extern "C" fn be_info_TestComponent()
    -> *const ComponentInfo {
    unsafe {
        &be_info_data_TestComponent as *const ComponentInfo
    }
}
#[no_mangle]
pub extern "C" fn be_clone_TestComponent(dest: *mut TestComponent,
                                         src: *const TestComponent) {
    unsafe { *dest = (*src).clone(); }
}
#[no_mangle]
pub extern "C" fn be_drop_TestComponent(component: *mut TestComponent) {
    unsafe { let data = Box::from_raw(component); drop(data); }
}
// ----------------- End expanded macros from brane_engine_api_macros -----------------

