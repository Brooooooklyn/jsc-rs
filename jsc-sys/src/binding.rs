#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const JSC_OBJC_API_ENABLED: u32 = 0;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContextGroup {
  _unused: [u8; 0],
}
pub type JSContextGroupRef = *const OpaqueJSContextGroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContext {
  _unused: [u8; 0],
}
pub type JSContextRef = *const OpaqueJSContext;
pub type JSGlobalContextRef = *mut OpaqueJSContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSString {
  _unused: [u8; 0],
}
pub type JSStringRef = *mut OpaqueJSString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSClass {
  _unused: [u8; 0],
}
pub type JSClassRef = *mut OpaqueJSClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameArray {
  _unused: [u8; 0],
}
pub type JSPropertyNameArrayRef = *mut OpaqueJSPropertyNameArray;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameAccumulator {
  _unused: [u8; 0],
}
pub type JSPropertyNameAccumulatorRef = *mut OpaqueJSPropertyNameAccumulator;
pub type JSTypedArrayBytesDeallocator = ::std::option::Option<
  unsafe extern "C" fn(
    bytes: *mut ::std::os::raw::c_void,
    deallocatorContext: *mut ::std::os::raw::c_void,
  ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSValue {
  _unused: [u8; 0],
}
pub type JSValueRef = *const OpaqueJSValue;
pub type JSObjectRef = *mut OpaqueJSValue;
extern "C" {
  pub fn JSEvaluateScript(
    ctx: JSContextRef,
    script: JSStringRef,
    thisObject: JSObjectRef,
    sourceURL: JSStringRef,
    startingLineNumber: ::std::os::raw::c_int,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
}
extern "C" {
  pub fn JSCheckScriptSyntax(
    ctx: JSContextRef,
    script: JSStringRef,
    sourceURL: JSStringRef,
    startingLineNumber: ::std::os::raw::c_int,
    exception: *mut JSValueRef,
  ) -> bool;
}
extern "C" {
  pub fn JSGarbageCollect(ctx: JSContextRef);
}
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub const JSType_kJSTypeUndefined: JSType = 0;
pub const JSType_kJSTypeNull: JSType = 1;
pub const JSType_kJSTypeBoolean: JSType = 2;
pub const JSType_kJSTypeNumber: JSType = 3;
pub const JSType_kJSTypeString: JSType = 4;
pub const JSType_kJSTypeObject: JSType = 5;
pub const JSType_kJSTypeSymbol: JSType = 6;
pub type JSType = u32;
pub const JSTypedArrayType_kJSTypedArrayTypeInt8Array: JSTypedArrayType = 0;
pub const JSTypedArrayType_kJSTypedArrayTypeInt16Array: JSTypedArrayType = 1;
pub const JSTypedArrayType_kJSTypedArrayTypeInt32Array: JSTypedArrayType = 2;
pub const JSTypedArrayType_kJSTypedArrayTypeUint8Array: JSTypedArrayType = 3;
pub const JSTypedArrayType_kJSTypedArrayTypeUint8ClampedArray: JSTypedArrayType = 4;
pub const JSTypedArrayType_kJSTypedArrayTypeUint16Array: JSTypedArrayType = 5;
pub const JSTypedArrayType_kJSTypedArrayTypeUint32Array: JSTypedArrayType = 6;
pub const JSTypedArrayType_kJSTypedArrayTypeFloat32Array: JSTypedArrayType = 7;
pub const JSTypedArrayType_kJSTypedArrayTypeFloat64Array: JSTypedArrayType = 8;
pub const JSTypedArrayType_kJSTypedArrayTypeArrayBuffer: JSTypedArrayType = 9;
pub const JSTypedArrayType_kJSTypedArrayTypeNone: JSTypedArrayType = 10;
pub const JSTypedArrayType_kJSTypedArrayTypeBigInt64Array: JSTypedArrayType = 11;
pub const JSTypedArrayType_kJSTypedArrayTypeBigUint64Array: JSTypedArrayType = 12;
pub type JSTypedArrayType = u32;
extern "C" {
  pub fn JSValueGetType(ctx: JSContextRef, value: JSValueRef) -> JSType;
}
extern "C" {
  pub fn JSValueIsUndefined(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsNull(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsNumber(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsString(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsSymbol(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsObject(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsObjectOfClass(ctx: JSContextRef, value: JSValueRef, jsClass: JSClassRef) -> bool;
}
extern "C" {
  pub fn JSValueIsArray(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsDate(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueGetTypedArrayType(
    ctx: JSContextRef,
    value: JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSTypedArrayType;
}
extern "C" {
  pub fn JSValueIsEqual(
    ctx: JSContextRef,
    a: JSValueRef,
    b: JSValueRef,
    exception: *mut JSValueRef,
  ) -> bool;
}
extern "C" {
  pub fn JSValueIsStrictEqual(ctx: JSContextRef, a: JSValueRef, b: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueIsInstanceOfConstructor(
    ctx: JSContextRef,
    value: JSValueRef,
    constructor: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> bool;
}
extern "C" {
  pub fn JSValueMakeUndefined(ctx: JSContextRef) -> JSValueRef;
}
extern "C" {
  pub fn JSValueMakeNull(ctx: JSContextRef) -> JSValueRef;
}
extern "C" {
  pub fn JSValueMakeBoolean(ctx: JSContextRef, boolean: bool) -> JSValueRef;
}
extern "C" {
  pub fn JSValueMakeNumber(ctx: JSContextRef, number: f64) -> JSValueRef;
}
extern "C" {
  pub fn JSValueMakeString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
}
extern "C" {
  pub fn JSValueMakeSymbol(ctx: JSContextRef, description: JSStringRef) -> JSValueRef;
}
extern "C" {
  pub fn JSValueMakeFromJSONString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
}
extern "C" {
  pub fn JSValueCreateJSONString(
    ctx: JSContextRef,
    value: JSValueRef,
    indent: ::std::os::raw::c_uint,
    exception: *mut JSValueRef,
  ) -> JSStringRef;
}
extern "C" {
  pub fn JSValueToBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
}
extern "C" {
  pub fn JSValueToNumber(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> f64;
}
extern "C" {
  pub fn JSValueToStringCopy(
    ctx: JSContextRef,
    value: JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSStringRef;
}
extern "C" {
  pub fn JSValueToObject(
    ctx: JSContextRef,
    value: JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSValueProtect(ctx: JSContextRef, value: JSValueRef);
}
extern "C" {
  pub fn JSValueUnprotect(ctx: JSContextRef, value: JSValueRef);
}
pub const kJSPropertyAttributeNone: _bindgen_ty_63 = 0;
pub const kJSPropertyAttributeReadOnly: _bindgen_ty_63 = 2;
pub const kJSPropertyAttributeDontEnum: _bindgen_ty_63 = 4;
pub const kJSPropertyAttributeDontDelete: _bindgen_ty_63 = 8;
pub type _bindgen_ty_63 = u32;
pub type JSPropertyAttributes = ::std::os::raw::c_uint;
pub const kJSClassAttributeNone: _bindgen_ty_64 = 0;
pub const kJSClassAttributeNoAutomaticPrototype: _bindgen_ty_64 = 2;
pub type _bindgen_ty_64 = u32;
pub type JSClassAttributes = ::std::os::raw::c_uint;
pub type JSObjectInitializeCallback =
  ::std::option::Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef)>;
pub type JSObjectFinalizeCallback =
  ::std::option::Option<unsafe extern "C" fn(object: JSObjectRef)>;
pub type JSObjectHasPropertyCallback = ::std::option::Option<
  unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> bool,
>;
pub type JSObjectGetPropertyCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyName: JSStringRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef,
>;
pub type JSObjectSetPropertyCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyName: JSStringRef,
    value: JSValueRef,
    exception: *mut JSValueRef,
  ) -> bool,
>;
pub type JSObjectDeletePropertyCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyName: JSStringRef,
    exception: *mut JSValueRef,
  ) -> bool,
>;
pub type JSObjectGetPropertyNamesCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyNames: JSPropertyNameAccumulatorRef,
  ),
>;
pub type JSObjectCallAsFunctionCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    function: JSObjectRef,
    thisObject: JSObjectRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef,
>;
pub type JSObjectCallAsConstructorCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    constructor: JSObjectRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef,
>;
pub type JSObjectHasInstanceCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    constructor: JSObjectRef,
    possibleInstance: JSValueRef,
    exception: *mut JSValueRef,
  ) -> bool,
>;
pub type JSObjectConvertToTypeCallback = ::std::option::Option<
  unsafe extern "C" fn(
    ctx: JSContextRef,
    object: JSObjectRef,
    type_: JSType,
    exception: *mut JSValueRef,
  ) -> JSValueRef,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticValue {
  pub name: *const ::std::os::raw::c_char,
  pub getProperty: JSObjectGetPropertyCallback,
  pub setProperty: JSObjectSetPropertyCallback,
  pub attributes: JSPropertyAttributes,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticFunction {
  pub name: *const ::std::os::raw::c_char,
  pub callAsFunction: JSObjectCallAsFunctionCallback,
  pub attributes: JSPropertyAttributes,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClassDefinition {
  pub version: ::std::os::raw::c_int,
  pub attributes: JSClassAttributes,
  pub className: *const ::std::os::raw::c_char,
  pub parentClass: JSClassRef,
  pub staticValues: *const JSStaticValue,
  pub staticFunctions: *const JSStaticFunction,
  pub initialize: JSObjectInitializeCallback,
  pub finalize: JSObjectFinalizeCallback,
  pub hasProperty: JSObjectHasPropertyCallback,
  pub getProperty: JSObjectGetPropertyCallback,
  pub setProperty: JSObjectSetPropertyCallback,
  pub deleteProperty: JSObjectDeletePropertyCallback,
  pub getPropertyNames: JSObjectGetPropertyNamesCallback,
  pub callAsFunction: JSObjectCallAsFunctionCallback,
  pub callAsConstructor: JSObjectCallAsConstructorCallback,
  pub hasInstance: JSObjectHasInstanceCallback,
  pub convertToType: JSObjectConvertToTypeCallback,
}
extern "C" {
  pub static kJSClassDefinitionEmpty: JSClassDefinition;
}
extern "C" {
  pub fn JSClassCreate(definition: *const JSClassDefinition) -> JSClassRef;
}
extern "C" {
  pub fn JSClassRetain(jsClass: JSClassRef) -> JSClassRef;
}
extern "C" {
  pub fn JSClassRelease(jsClass: JSClassRef);
}
extern "C" {
  pub fn JSObjectMake(
    ctx: JSContextRef,
    jsClass: JSClassRef,
    data: *mut ::std::os::raw::c_void,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeFunctionWithCallback(
    ctx: JSContextRef,
    name: JSStringRef,
    callAsFunction: JSObjectCallAsFunctionCallback,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeConstructor(
    ctx: JSContextRef,
    jsClass: JSClassRef,
    callAsConstructor: JSObjectCallAsConstructorCallback,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeArray(
    ctx: JSContextRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeDate(
    ctx: JSContextRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeError(
    ctx: JSContextRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeRegExp(
    ctx: JSContextRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeDeferredPromise(
    ctx: JSContextRef,
    resolve: *mut JSObjectRef,
    reject: *mut JSObjectRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeFunction(
    ctx: JSContextRef,
    name: JSStringRef,
    parameterCount: ::std::os::raw::c_uint,
    parameterNames: *const JSStringRef,
    body: JSStringRef,
    sourceURL: JSStringRef,
    startingLineNumber: ::std::os::raw::c_int,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectGetPrototype(ctx: JSContextRef, object: JSObjectRef) -> JSValueRef;
}
extern "C" {
  pub fn JSObjectSetPrototype(ctx: JSContextRef, object: JSObjectRef, value: JSValueRef);
}
extern "C" {
  pub fn JSObjectHasProperty(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyName: JSStringRef,
  ) -> bool;
}
extern "C" {
  pub fn JSObjectGetProperty(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyName: JSStringRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
}
extern "C" {
  pub fn JSObjectSetProperty(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyName: JSStringRef,
    value: JSValueRef,
    attributes: JSPropertyAttributes,
    exception: *mut JSValueRef,
  );
}
extern "C" {
  pub fn JSObjectDeleteProperty(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyName: JSStringRef,
    exception: *mut JSValueRef,
  ) -> bool;
}
extern "C" {
  pub fn JSObjectHasPropertyForKey(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyKey: JSValueRef,
    exception: *mut JSValueRef,
  ) -> bool;
}
extern "C" {
  pub fn JSObjectGetPropertyForKey(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyKey: JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
}
extern "C" {
  pub fn JSObjectSetPropertyForKey(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyKey: JSValueRef,
    value: JSValueRef,
    attributes: JSPropertyAttributes,
    exception: *mut JSValueRef,
  );
}
extern "C" {
  pub fn JSObjectDeletePropertyForKey(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyKey: JSValueRef,
    exception: *mut JSValueRef,
  ) -> bool;
}
extern "C" {
  pub fn JSObjectGetPropertyAtIndex(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyIndex: ::std::os::raw::c_uint,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
}
extern "C" {
  pub fn JSObjectSetPropertyAtIndex(
    ctx: JSContextRef,
    object: JSObjectRef,
    propertyIndex: ::std::os::raw::c_uint,
    value: JSValueRef,
    exception: *mut JSValueRef,
  );
}
extern "C" {
  pub fn JSObjectGetPrivate(object: JSObjectRef) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn JSObjectSetPrivate(object: JSObjectRef, data: *mut ::std::os::raw::c_void) -> bool;
}
extern "C" {
  pub fn JSObjectIsFunction(ctx: JSContextRef, object: JSObjectRef) -> bool;
}
extern "C" {
  pub fn JSObjectCallAsFunction(
    ctx: JSContextRef,
    object: JSObjectRef,
    thisObject: JSObjectRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef;
}
extern "C" {
  pub fn JSObjectIsConstructor(ctx: JSContextRef, object: JSObjectRef) -> bool;
}
extern "C" {
  pub fn JSObjectCallAsConstructor(
    ctx: JSContextRef,
    object: JSObjectRef,
    argumentCount: usize,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectCopyPropertyNames(
    ctx: JSContextRef,
    object: JSObjectRef,
  ) -> JSPropertyNameArrayRef;
}
extern "C" {
  pub fn JSPropertyNameArrayRetain(array: JSPropertyNameArrayRef) -> JSPropertyNameArrayRef;
}
extern "C" {
  pub fn JSPropertyNameArrayRelease(array: JSPropertyNameArrayRef);
}
extern "C" {
  pub fn JSPropertyNameArrayGetCount(array: JSPropertyNameArrayRef) -> usize;
}
extern "C" {
  pub fn JSPropertyNameArrayGetNameAtIndex(
    array: JSPropertyNameArrayRef,
    index: usize,
  ) -> JSStringRef;
}
extern "C" {
  pub fn JSPropertyNameAccumulatorAddName(
    accumulator: JSPropertyNameAccumulatorRef,
    propertyName: JSStringRef,
  );
}
extern "C" {
  pub fn JSContextGroupCreate() -> JSContextGroupRef;
}
extern "C" {
  pub fn JSContextGroupRetain(group: JSContextGroupRef) -> JSContextGroupRef;
}
extern "C" {
  pub fn JSContextGroupRelease(group: JSContextGroupRef);
}
extern "C" {
  pub fn JSGlobalContextCreate(globalObjectClass: JSClassRef) -> JSGlobalContextRef;
}
extern "C" {
  pub fn JSGlobalContextCreateInGroup(
    group: JSContextGroupRef,
    globalObjectClass: JSClassRef,
  ) -> JSGlobalContextRef;
}
extern "C" {
  pub fn JSGlobalContextRetain(ctx: JSGlobalContextRef) -> JSGlobalContextRef;
}
extern "C" {
  pub fn JSGlobalContextRelease(ctx: JSGlobalContextRef);
}
extern "C" {
  pub fn JSContextGetGlobalObject(ctx: JSContextRef) -> JSObjectRef;
}
extern "C" {
  pub fn JSContextGetGroup(ctx: JSContextRef) -> JSContextGroupRef;
}
extern "C" {
  pub fn JSContextGetGlobalContext(ctx: JSContextRef) -> JSGlobalContextRef;
}
extern "C" {
  pub fn JSGlobalContextCopyName(ctx: JSGlobalContextRef) -> JSStringRef;
}
extern "C" {
  pub fn JSGlobalContextSetName(ctx: JSGlobalContextRef, name: JSStringRef);
}
pub type JSChar = ::std::os::raw::c_ushort;
extern "C" {
  pub fn JSStringCreateWithCharacters(chars: *const JSChar, numChars: usize) -> JSStringRef;
}
extern "C" {
  pub fn JSStringCreateWithUTF8CString(string: *const ::std::os::raw::c_char) -> JSStringRef;
}
extern "C" {
  pub fn JSStringRetain(string: JSStringRef) -> JSStringRef;
}
extern "C" {
  pub fn JSStringRelease(string: JSStringRef);
}
extern "C" {
  pub fn JSStringGetLength(string: JSStringRef) -> usize;
}
extern "C" {
  pub fn JSStringGetCharactersPtr(string: JSStringRef) -> *const JSChar;
}
extern "C" {
  pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> usize;
}
extern "C" {
  pub fn JSStringGetUTF8CString(
    string: JSStringRef,
    buffer: *mut ::std::os::raw::c_char,
    bufferSize: usize,
  ) -> usize;
}
extern "C" {
  pub fn JSStringIsEqual(a: JSStringRef, b: JSStringRef) -> bool;
}
extern "C" {
  pub fn JSStringIsEqualToUTF8CString(a: JSStringRef, b: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
  pub fn JSObjectMakeTypedArray(
    ctx: JSContextRef,
    arrayType: JSTypedArrayType,
    length: usize,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeTypedArrayWithBytesNoCopy(
    ctx: JSContextRef,
    arrayType: JSTypedArrayType,
    bytes: *mut ::std::os::raw::c_void,
    byteLength: usize,
    bytesDeallocator: JSTypedArrayBytesDeallocator,
    deallocatorContext: *mut ::std::os::raw::c_void,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeTypedArrayWithArrayBuffer(
    ctx: JSContextRef,
    arrayType: JSTypedArrayType,
    buffer: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(
    ctx: JSContextRef,
    arrayType: JSTypedArrayType,
    buffer: JSObjectRef,
    byteOffset: usize,
    length: usize,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectGetTypedArrayBytesPtr(
    ctx: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn JSObjectGetTypedArrayLength(
    ctx: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> usize;
}
extern "C" {
  pub fn JSObjectGetTypedArrayByteLength(
    ctx: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> usize;
}
extern "C" {
  pub fn JSObjectGetTypedArrayByteOffset(
    ctx: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> usize;
}
extern "C" {
  pub fn JSObjectGetTypedArrayBuffer(
    ctx: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectMakeArrayBufferWithBytesNoCopy(
    ctx: JSContextRef,
    bytes: *mut ::std::os::raw::c_void,
    byteLength: usize,
    bytesDeallocator: JSTypedArrayBytesDeallocator,
    deallocatorContext: *mut ::std::os::raw::c_void,
    exception: *mut JSValueRef,
  ) -> JSObjectRef;
}
extern "C" {
  pub fn JSObjectGetArrayBufferBytesPtr(
    ctx: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn JSObjectGetArrayBufferByteLength(
    ctx: JSContextRef,
    object: JSObjectRef,
    exception: *mut JSValueRef,
  ) -> usize;
}
