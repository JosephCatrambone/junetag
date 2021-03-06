use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct apriltag_family {
    pub ncodes: uint32_t,
    pub codes: *mut uint64_t,
    pub width_at_border: libc::c_int,
    pub total_width: libc::c_int,
    pub reversed_border: bool,
    pub nbits: uint32_t,
    pub bit_x: *mut uint32_t,
    pub bit_y: *mut uint32_t,
    pub h: uint32_t,
    pub name: *mut libc::c_char,
    pub impl_0: *mut libc::c_void,
}
pub type apriltag_family_t = apriltag_family;
static mut codedata: [uint64_t; 587] = [
    0xd7e00984b as libc::c_ulong,
    0xdda664ca7 as libc::c_ulong,
    0xdc4a1c821 as libc::c_ulong,
    0xe17b470e9 as libc::c_ulong,
    0xef91d01b1 as libc::c_ulong,
    0xf429cdd73 as libc::c_ulong,
    0x5da29225 as libc::c_ulong,
    0x1106cba43 as libc::c_ulong,
    0x223bed79d as libc::c_ulong,
    0x21f51213c as libc::c_ulong,
    0x33eb19ca6 as libc::c_ulong,
    0x3f76eb0f8 as libc::c_ulong,
    0x469a97414 as libc::c_ulong,
    0x45dcfe0b0 as libc::c_ulong,
    0x4a6465f72 as libc::c_ulong,
    0x51801db96 as libc::c_ulong,
    0x5eb946b4e as libc::c_ulong,
    0x68a7cc2ec as libc::c_ulong,
    0x6f0ba2652 as libc::c_ulong,
    0x78765559d as libc::c_ulong,
    0x87b83d129 as libc::c_ulong,
    0x86cc4a5c5 as libc::c_ulong,
    0x8b64df90f as libc::c_ulong,
    0x9c577b611 as libc::c_ulong,
    0xa3810f2f5 as libc::c_ulong,
    0xaf4d75b83 as libc::c_ulong,
    0xb59a03fef as libc::c_ulong,
    0xbb1096f85 as libc::c_ulong,
    0xd1b92fc76 as libc::c_ulong,
    0xd0dd509d2 as libc::c_ulong,
    0xe2cfda160 as libc::c_ulong,
    0x2ff497c63 as libc::c_ulong,
    0x47240671b as libc::c_ulong,
    0x5047a2e55 as libc::c_ulong,
    0x635ca87c7 as libc::c_ulong,
    0x691254166 as libc::c_ulong,
    0x68f43d94a as libc::c_ulong,
    0x6ef24bdb6 as libc::c_ulong,
    0x8cdd8f886 as libc::c_ulong,
    0x9de96b718 as libc::c_ulong,
    0xaff6e5a8a as libc::c_ulong,
    0xbae46f029 as libc::c_ulong,
    0xd225b6d59 as libc::c_ulong,
    0xdf8ba8c01 as libc::c_ulong,
    0xe3744a22f as libc::c_ulong,
    0xfbb59375d as libc::c_ulong,
    0x18a916828 as libc::c_ulong,
    0x22f29c1ba as libc::c_ulong,
    0x286887d58 as libc::c_ulong,
    0x41392322e as libc::c_ulong,
    0x75d18ecd1 as libc::c_ulong,
    0x87c302743 as libc::c_ulong,
    0x8c6317ba9 as libc::c_ulong,
    0x9e40f36d7 as libc::c_ulong,
    0xc0e5a806a as libc::c_ulong,
    0xcc78cb87c as libc::c_ulong,
    0x12d2f2d01 as libc::c_ulong,
    0x379f36a21 as libc::c_ulong,
    0x6973f59ac as libc::c_ulong,
    0x7789ea9f4 as libc::c_ulong,
    0x8f1c73e84 as libc::c_ulong,
    0x8dd287a20 as libc::c_ulong,
    0x94a4eee4c as libc::c_ulong,
    0xa455379b5 as libc::c_ulong,
    0xa9e92987d as libc::c_ulong,
    0xbd25cb40b as libc::c_ulong,
    0xbe98d3582 as libc::c_ulong,
    0xd3d5972b2 as libc::c_ulong,
    0x14c53d7c7 as libc::c_ulong,
    0x4f1796936 as libc::c_ulong,
    0x4e71fed1a as libc::c_ulong,
    0x66d46fae0 as libc::c_ulong,
    0xa55abb933 as libc::c_ulong,
    0xebee1acca as libc::c_ulong,
    0x1ad4ba6a4 as libc::c_ulong,
    0x305b17571 as libc::c_ulong,
    0x553611351 as libc::c_ulong,
    0x59ca62775 as libc::c_ulong,
    0x7819cb6a1 as libc::c_ulong,
    0xedb7bc9eb as libc::c_ulong,
    0x5b2694212 as libc::c_ulong,
    0x72e12d185 as libc::c_ulong,
    0xed6152e2c as libc::c_ulong,
    0x5bcdadbf3 as libc::c_ulong,
    0x78e0aa0c6 as libc::c_ulong,
    0xc60a0b909 as libc::c_ulong,
    0xef9a34b0d as libc::c_ulong,
    0x398a6621a as libc::c_ulong,
    0xa8a27c944 as libc::c_ulong,
    0x4b564304e as libc::c_ulong,
    0x52902b4e2 as libc::c_ulong,
    0x857280b56 as libc::c_ulong,
    0xa91b2c84b as libc::c_ulong,
    0xe91df939b as libc::c_ulong,
    0x1fa405f28 as libc::c_ulong,
    0x23793ab86 as libc::c_ulong,
    0x68c17729f as libc::c_ulong,
    0x9fbf3b840 as libc::c_ulong,
    0x36922413c as libc::c_ulong,
    0x4eb5f946e as libc::c_ulong,
    0x533fe2404 as libc::c_ulong,
    0x63de7d35e as libc::c_ulong,
    0x925eddc72 as libc::c_ulong,
    0x99b8b3896 as libc::c_ulong,
    0xaace4c708 as libc::c_ulong,
    0xc22994af0 as libc::c_ulong,
    0x8f1eae41b as libc::c_ulong,
    0xd95fb486c as libc::c_ulong,
    0x13fb77857 as libc::c_ulong,
    0x4fe0983a3 as libc::c_ulong,
    0xd559bf8a9 as libc::c_ulong,
    0xe1855d78d as libc::c_ulong,
    0xfec8daaad as libc::c_ulong,
    0x71ecb6d95 as libc::c_ulong,
    0xdc9e50e4c as libc::c_ulong,
    0xca3a4c259 as libc::c_ulong,
    0x740d12bbf as libc::c_ulong,
    0xaeedd18e0 as libc::c_ulong,
    0xb509b9c8e as libc::c_ulong,
    0x5232fea1c as libc::c_ulong,
    0x19282d18b as libc::c_ulong,
    0x76c22d67b as libc::c_ulong,
    0x936beb34b as libc::c_ulong,
    0x8a5ea8dd as libc::c_ulong,
    0x679eadc28 as libc::c_ulong,
    0xa08e119c5 as libc::c_ulong,
    0x20a6e3e24 as libc::c_ulong,
    0x7eab9c239 as libc::c_ulong,
    0x96632c32e as libc::c_ulong,
    0x470d06e44 as libc::c_ulong,
    0x8a70212fb as libc::c_ulong,
    0xa7e4251b as libc::c_ulong,
    0x9ec762cc0 as libc::c_ulong,
    0xd8a3a1f48 as libc::c_ulong,
    0xdb680f346 as libc::c_ulong,
    0x4a1e93a9d as libc::c_ulong,
    0x638ddc04f as libc::c_ulong,
    0x4c2fcc993 as libc::c_ulong,
    0x1ef28c95 as libc::c_ulong,
    0xbf0d9792d as libc::c_ulong,
    0x6d27557c3 as libc::c_ulong,
    0x623f977f4 as libc::c_ulong,
    0x35b43be57 as libc::c_ulong,
    0xbb0c428d5 as libc::c_ulong,
    0xa6f01474d as libc::c_ulong,
    0x5a70c9749 as libc::c_ulong,
    0x20ddabc3b as libc::c_ulong,
    0x2eabd78cf as libc::c_ulong,
    0x90aa18f88 as libc::c_ulong,
    0xa9ea89350 as libc::c_ulong,
    0x3cdb39b22 as libc::c_ulong,
    0x839a08f34 as libc::c_ulong,
    0x169bb814e as libc::c_ulong,
    0x1a575ab08 as libc::c_ulong,
    0xa04d3d5a2 as libc::c_ulong,
    0xbf7902f2b as libc::c_ulong,
    0x95a5e65c as libc::c_ulong,
    0x92e8fce94 as libc::c_ulong,
    0x67ef48d12 as libc::c_ulong,
    0x6400dbcac as libc::c_ulong,
    0xb12d8fb9f as libc::c_ulong,
    0x347f45d3 as libc::c_ulong,
    0xb35826f56 as libc::c_ulong,
    0xc546ac6e4 as libc::c_ulong,
    0x81cc35b66 as libc::c_ulong,
    0x41d14bd57 as libc::c_ulong,
    0xc052b168 as libc::c_ulong,
    0x7d6ce5018 as libc::c_ulong,
    0xab4ed5ede as libc::c_ulong,
    0x5af817119 as libc::c_ulong,
    0xd1454b182 as libc::c_ulong,
    0x2badb090b as libc::c_ulong,
    0x3fcb4c0c as libc::c_ulong,
    0x2f1c28fd8 as libc::c_ulong,
    0x93608c6f7 as libc::c_ulong,
    0x4c93ba2b5 as libc::c_ulong,
    0x7d950a5d as libc::c_ulong,
    0xe54b3d3fc as libc::c_ulong,
    0x15560cf9d as libc::c_ulong,
    0x189e4958a as libc::c_ulong,
    0x62140e9d2 as libc::c_ulong,
    0x723bc1cdb as libc::c_ulong,
    0x2063f26fa as libc::c_ulong,
    0xfa08ab19f as libc::c_ulong,
    0x7955641db as libc::c_ulong,
    0x646b01daa as libc::c_ulong,
    0x71cd427cc as libc::c_ulong,
    0x9a42f7d4 as libc::c_ulong,
    0x717edc643 as libc::c_ulong,
    0x15eb94367 as libc::c_ulong,
    0x8392e6bb2 as libc::c_ulong,
    0x832408542 as libc::c_ulong,
    0x2b9b874be as libc::c_ulong,
    0xb21f4730d as libc::c_ulong,
    0xb5d8f24c9 as libc::c_ulong,
    0x7dbaf6931 as libc::c_ulong,
    0x1b4e33629 as libc::c_ulong,
    0x13452e710 as libc::c_ulong,
    0xe974af612 as libc::c_ulong,
    0x1df61d29a as libc::c_ulong,
    0x99f2532ad as libc::c_ulong,
    0xe50ec71b4 as libc::c_ulong,
    0x5df0a36e8 as libc::c_ulong,
    0x4934e4cea as libc::c_ulong,
    0xe34a0b4bd as libc::c_ulong,
    0xb7b26b588 as libc::c_ulong,
    0xf255118d as libc::c_ulong,
    0xd0c8fa31e as libc::c_ulong,
    0x6a50c94f as libc::c_ulong,
    0xf28aa9f06 as libc::c_ulong,
    0x131d194d8 as libc::c_ulong,
    0x622e3da79 as libc::c_ulong,
    0xac7478303 as libc::c_ulong,
    0xc8f2521d7 as libc::c_ulong,
    0x6c9c881f5 as libc::c_ulong,
    0x49e38b60a as libc::c_ulong,
    0x513d8df65 as libc::c_ulong,
    0xd7c2b0785 as libc::c_ulong,
    0x9f6f9d75a as libc::c_ulong,
    0x9f6966020 as libc::c_ulong,
    0x1e1a54e33 as libc::c_ulong,
    0xc04d63419 as libc::c_ulong,
    0x946e04cd7 as libc::c_ulong,
    0x1bdac5902 as libc::c_ulong,
    0x56469b830 as libc::c_ulong,
    0xffad59569 as libc::c_ulong,
    0x86970e7d8 as libc::c_ulong,
    0x8a4b41e12 as libc::c_ulong,
    0xad4688e3b as libc::c_ulong,
    0x85f8f5df4 as libc::c_ulong,
    0xd833a0893 as libc::c_ulong,
    0x2a36fdd7c as libc::c_ulong,
    0xd6a857cf2 as libc::c_ulong,
    0x8829bc35c as libc::c_ulong,
    0x5e50d79bc as libc::c_ulong,
    0xfbb8035e4 as libc::c_ulong,
    0xc1a95bebf as libc::c_ulong,
    0x36b0baf8 as libc::c_ulong,
    0xe0da964ea as libc::c_ulong,
    0xb6483689b as libc::c_ulong,
    0x7c8e2f4c1 as libc::c_ulong,
    0x5b856a23b as libc::c_ulong,
    0x2fc183995 as libc::c_ulong,
    0xe914b6d70 as libc::c_ulong,
    0xb31041969 as libc::c_ulong,
    0x1bb478493 as libc::c_ulong,
    0x63e2b456 as libc::c_ulong,
    0xf2a082b9c as libc::c_ulong,
    0x8e5e646ea as libc::c_ulong,
    0x8172f8f6 as libc::c_ulong,
    0xdacd923e as libc::c_ulong,
    0xe5dcf0e2e as libc::c_ulong,
    0xbf9446bae as libc::c_ulong,
    0x4822d50d1 as libc::c_ulong,
    0x26e710bf5 as libc::c_ulong,
    0xb90ba2a24 as libc::c_ulong,
    0xf3b25aa73 as libc::c_ulong,
    0x809ad589b as libc::c_ulong,
    0x94cc1e254 as libc::c_ulong,
    0x5334a3adb as libc::c_ulong,
    0x592886b2f as libc::c_ulong,
    0xbf64704aa as libc::c_ulong,
    0x566dbf24c as libc::c_ulong,
    0x72203e692 as libc::c_ulong,
    0x64e61e809 as libc::c_ulong,
    0xd7259aad6 as libc::c_ulong,
    0x7b924aedc as libc::c_ulong,
    0x2df2184e8 as libc::c_ulong,
    0x353d1eca7 as libc::c_ulong,
    0xfce30d7ce as libc::c_ulong,
    0xf7b0f436e as libc::c_ulong,
    0x57e8d8f68 as libc::c_ulong,
    0x8c79e60db as libc::c_ulong,
    0x9c8362b2b as libc::c_ulong,
    0x63a5804f2 as libc::c_ulong,
    0x9298353dc as libc::c_ulong,
    0x6f98a71c8 as libc::c_ulong,
    0xa5731f693 as libc::c_ulong,
    0x21ca5c870 as libc::c_ulong,
    0x1c2107fd3 as libc::c_ulong,
    0x6181f6c39 as libc::c_ulong,
    0x19e574304 as libc::c_ulong,
    0x329937606 as libc::c_ulong,
    0x43d5c70d as libc::c_ulong,
    0x9b18ff162 as libc::c_ulong,
    0x8e2ccfebf as libc::c_ulong,
    0x72b7b9b54 as libc::c_ulong,
    0x9b71f4f3c as libc::c_ulong,
    0x935d7393e as libc::c_ulong,
    0x65938881a as libc::c_ulong,
    0x6a5bd6f2d as libc::c_ulong,
    0xa19783306 as libc::c_ulong,
    0xe6472f4d7 as libc::c_ulong,
    0x81163df5a as libc::c_ulong,
    0xa838e1cbd as libc::c_ulong,
    0x982748477 as libc::c_ulong,
    0x50c54feb as libc::c_ulong,
    0xd82fbb58 as libc::c_ulong,
    0x2c4c72799 as libc::c_ulong,
    0x97d259ad6 as libc::c_ulong,
    0x22d9a43ed as libc::c_ulong,
    0xfdb162a9f as libc::c_ulong,
    0xcb4a727d as libc::c_ulong,
    0x4fae2e371 as libc::c_ulong,
    0x535b5be8b as libc::c_ulong,
    0x48795908a as libc::c_ulong,
    0xce7c18962 as libc::c_ulong,
    0x4ea154d80 as libc::c_ulong,
    0x50c064889 as libc::c_ulong,
    0x8d97fc75d as libc::c_ulong,
    0xc8bd9ec61 as libc::c_ulong,
    0x83ee8e8bb as libc::c_ulong,
    0xc8431419a as libc::c_ulong,
    0x1aa78079d as libc::c_ulong,
    0x8111aa4a5 as libc::c_ulong,
    0xdfa3a69fe as libc::c_ulong,
    0x51630d83f as libc::c_ulong,
    0x2d930fb3f as libc::c_ulong,
    0x2133116e5 as libc::c_ulong,
    0xae5395522 as libc::c_ulong,
    0xbc07a4e8a as libc::c_ulong,
    0x57bf08ba0 as libc::c_ulong,
    0x6cb18036a as libc::c_ulong,
    0xf0e2e4b75 as libc::c_ulong,
    0x3eb692b6f as libc::c_ulong,
    0xd8178a3fa as libc::c_ulong,
    0x238cce6a6 as libc::c_ulong,
    0xe97d5cdd7 as libc::c_ulong,
    0xfe10d8d5e as libc::c_ulong,
    0xb39584a1d as libc::c_ulong,
    0xca03536fd as libc::c_ulong,
    0xaa61f3998 as libc::c_ulong,
    0x72ff23ec2 as libc::c_ulong,
    0x15aa7d770 as libc::c_ulong,
    0x57a3a1282 as libc::c_ulong,
    0xd1f3902dc as libc::c_ulong,
    0x6554c9388 as libc::c_ulong,
    0xfd01283c7 as libc::c_ulong,
    0xe8baa42c5 as libc::c_ulong,
    0x72cee6adf as libc::c_ulong,
    0xf6614b3fa as libc::c_ulong,
    0x95c3778a2 as libc::c_ulong,
    0x7da4cea7a as libc::c_ulong,
    0xd18a5912c as libc::c_ulong,
    0xd116426e5 as libc::c_ulong,
    0x27c17bc1c as libc::c_ulong,
    0xb95b53bc1 as libc::c_ulong,
    0xc8f937a05 as libc::c_ulong,
    0xed220c9bd as libc::c_ulong,
    0xc97d72ab as libc::c_ulong,
    0x8fb1217ae as libc::c_ulong,
    0x25ca8a5a1 as libc::c_ulong,
    0xb261b871b as libc::c_ulong,
    0x1bef0a056 as libc::c_ulong,
    0x806a51179 as libc::c_ulong,
    0xeed249145 as libc::c_ulong,
    0x3f82aeceb as libc::c_ulong,
    0xcc56e9acf as libc::c_ulong,
    0x2e78d01eb as libc::c_ulong,
    0x102cee17f as libc::c_ulong,
    0x37caad3d5 as libc::c_ulong,
    0x16ac5b1ee as libc::c_ulong,
    0x2af164ece as libc::c_ulong,
    0xd4cd81dc9 as libc::c_ulong,
    0x12263a7e7 as libc::c_ulong,
    0x57ac7d117 as libc::c_ulong,
    0x9391d9740 as libc::c_ulong,
    0x7aedaa77f as libc::c_ulong,
    0x9675a3c72 as libc::c_ulong,
    0x277f25191 as libc::c_ulong,
    0xebb6e64b9 as libc::c_ulong,
    0x7ad3ef747 as libc::c_ulong,
    0x12759b181 as libc::c_ulong,
    0x948257d4d as libc::c_ulong,
    0xb63a850f6 as libc::c_ulong,
    0x3a52a8f75 as libc::c_ulong,
    0x4a019532c as libc::c_ulong,
    0xa021a7529 as libc::c_ulong,
    0xcc661876d as libc::c_ulong,
    0x4085afd05 as libc::c_ulong,
    0xe7048e089 as libc::c_ulong,
    0x3f979cdc6 as libc::c_ulong,
    0xd9da9071b as libc::c_ulong,
    0xed2fc5b68 as libc::c_ulong,
    0x79d64c3a1 as libc::c_ulong,
    0xfd44e2361 as libc::c_ulong,
    0x8eea46a74 as libc::c_ulong,
    0x42233b9c2 as libc::c_ulong,
    0xae4d1765d as libc::c_ulong,
    0x7303a094c as libc::c_ulong,
    0x2d7033abe as libc::c_ulong,
    0x3dcc2b0b4 as libc::c_ulong,
    0xf0967d09 as libc::c_ulong,
    0x6f0cd7de as libc::c_ulong,
    0x9807aca0 as libc::c_ulong,
    0x3a295cad3 as libc::c_ulong,
    0x2b106b202 as libc::c_ulong,
    0x3f38a828e as libc::c_ulong,
    0x78af46596 as libc::c_ulong,
    0xbda2dc713 as libc::c_ulong,
    0x9a8c8c9d9 as libc::c_ulong,
    0x6a0f2ddce as libc::c_ulong,
    0xa76af6fe2 as libc::c_ulong,
    0x86f66fa4 as libc::c_ulong,
    0xd52d63f8d as libc::c_ulong,
    0x89f7a6e73 as libc::c_ulong,
    0xcc6b23362 as libc::c_ulong,
    0xb4ebf3c39 as libc::c_ulong,
    0x564f300fa as libc::c_ulong,
    0xe8de3a706 as libc::c_ulong,
    0x79a033b61 as libc::c_ulong,
    0x765e160c5 as libc::c_ulong,
    0xa266a4f85 as libc::c_ulong,
    0xa68c38c24 as libc::c_ulong,
    0xdca0711fb as libc::c_ulong,
    0x85fba85ba as libc::c_ulong,
    0x37a207b46 as libc::c_ulong,
    0x158fcc4d0 as libc::c_ulong,
    0x569d79b3 as libc::c_ulong,
    0x7b1a25555 as libc::c_ulong,
    0xa8ae22468 as libc::c_ulong,
    0x7c592bdfd as libc::c_ulong,
    0xc59a5f66 as libc::c_ulong,
    0xb1115daa3 as libc::c_ulong,
    0xf17c87177 as libc::c_ulong,
    0x6769d766b as libc::c_ulong,
    0x2b637356d as libc::c_ulong,
    0x13d8685ac as libc::c_ulong,
    0xf24cb6ec0 as libc::c_ulong,
    0xbd0b56d1 as libc::c_ulong,
    0x42ff0e26d as libc::c_ulong,
    0xb41609267 as libc::c_ulong,
    0x96f9518af as libc::c_ulong,
    0xc56f96636 as libc::c_ulong,
    0x4a8e10349 as libc::c_ulong,
    0x863512171 as libc::c_ulong,
    0xea455d86c as libc::c_ulong,
    0xbd0e25279 as libc::c_ulong,
    0xe65e3f761 as libc::c_ulong,
    0x36c84a922 as libc::c_ulong,
    0x85fd1b38f as libc::c_ulong,
    0x657c91539 as libc::c_ulong,
    0x15033fe04 as libc::c_ulong,
    0x9051c921 as libc::c_ulong,
    0xab27d80d8 as libc::c_ulong,
    0xf92f7d0a1 as libc::c_ulong,
    0x8eb6bb737 as libc::c_ulong,
    0x10b5b0f63 as libc::c_ulong,
    0x6c9c7ad63 as libc::c_ulong,
    0xf66fe70ae as libc::c_ulong,
    0xca579bd92 as libc::c_ulong,
    0x956198e4d as libc::c_ulong,
    0x29e4405e5 as libc::c_ulong,
    0xe44eb885c as libc::c_ulong,
    0x41612456c as libc::c_ulong,
    0xea45e0abf as libc::c_ulong,
    0xd326529bd as libc::c_ulong,
    0x7b2c33cef as libc::c_ulong,
    0x80bc9b558 as libc::c_ulong,
    0x7169b9740 as libc::c_ulong,
    0xc37f99209 as libc::c_ulong,
    0x31ff6dab9 as libc::c_ulong,
    0xc795190ed as libc::c_ulong,
    0xa7636e95f as libc::c_ulong,
    0x9df075841 as libc::c_ulong,
    0x55a083932 as libc::c_ulong,
    0xa7cbdf630 as libc::c_ulong,
    0x409ea4ef0 as libc::c_ulong,
    0x92a1991b6 as libc::c_ulong,
    0x4b078dee9 as libc::c_ulong,
    0xae18ce9e4 as libc::c_ulong,
    0x5a6e1ef35 as libc::c_ulong,
    0x1a403bd59 as libc::c_ulong,
    0x31ea70a83 as libc::c_ulong,
    0x2bc3c4f3a as libc::c_ulong,
    0x5c921b3cb as libc::c_ulong,
    0x42da05c5 as libc::c_ulong,
    0x1f667d16b as libc::c_ulong,
    0x416a368cf as libc::c_ulong,
    0xfbc0a7a3b as libc::c_ulong,
    0x9419f0c7c as libc::c_ulong,
    0x81be2fa03 as libc::c_ulong,
    0x34e2c172f as libc::c_ulong,
    0x28648d8ae as libc::c_ulong,
    0xc7acbb885 as libc::c_ulong,
    0x45f31eb6a as libc::c_ulong,
    0xd1cfc0a7b as libc::c_ulong,
    0x42c4d260d as libc::c_ulong,
    0xcf6584097 as libc::c_ulong,
    0x94b132b14 as libc::c_ulong,
    0x3c5c5df75 as libc::c_ulong,
    0x8ae596fef as libc::c_ulong,
    0xaea8054eb as libc::c_ulong,
    0xae9cc573 as libc::c_ulong,
    0x496fb731b as libc::c_ulong,
    0xebf105662 as libc::c_ulong,
    0xaf9c83a37 as libc::c_ulong,
    0xc0d64cd6b as libc::c_ulong,
    0x7b608159a as libc::c_ulong,
    0xe74431642 as libc::c_ulong,
    0xd6fb9d900 as libc::c_ulong,
    0x291e99de0 as libc::c_ulong,
    0x10500ba9a as libc::c_ulong,
    0x5cd05d037 as libc::c_ulong,
    0xa87254fb2 as libc::c_ulong,
    0x9d7824a37 as libc::c_ulong,
    0x8b2c7b47c as libc::c_ulong,
    0x30c788145 as libc::c_ulong,
    0x2f4e5a8be as libc::c_ulong,
    0xbadb884da as libc::c_ulong,
    0x26e0d5c9 as libc::c_ulong,
    0x6fdbaa32e as libc::c_ulong,
    0x34758eb31 as libc::c_ulong,
    0x565cd1b4f as libc::c_ulong,
    0x2bfd90fb0 as libc::c_ulong,
    0x93052a6b as libc::c_ulong,
    0xd3c13c4b9 as libc::c_ulong,
    0x2daea43bf as libc::c_ulong,
    0xa279762bc as libc::c_ulong,
    0xf1bd9f22c as libc::c_ulong,
    0x4b7fec94f as libc::c_ulong,
    0x545761d5a as libc::c_ulong,
    0x7327df411 as libc::c_ulong,
    0x1b52a442e as libc::c_ulong,
    0x49b0ce108 as libc::c_ulong,
    0x24c764bc8 as libc::c_ulong,
    0x374563045 as libc::c_ulong,
    0xa3e8f91c6 as libc::c_ulong,
    0xe6bd2241 as libc::c_ulong,
    0xe0e52ee3c as libc::c_ulong,
    0x7e8e3caa as libc::c_ulong,
    0x96c2b7372 as libc::c_ulong,
    0x33acbdfda as libc::c_ulong,
    0xb15d91e54 as libc::c_ulong,
    0x464759ac1 as libc::c_ulong,
    0x6886a1998 as libc::c_ulong,
    0x57f5d3958 as libc::c_ulong,
    0x5a1f5c1f5 as libc::c_ulong,
    0xb58158ad as libc::c_ulong,
    0xe712053fb as libc::c_ulong,
    0x5352ddb25 as libc::c_ulong,
    0x414b98ea0 as libc::c_ulong,
    0x74f89f546 as libc::c_ulong,
    0x38a56b3c3 as libc::c_ulong,
    0x38db0dc17 as libc::c_ulong,
    0xaa016a755 as libc::c_ulong,
    0xdc72366f5 as libc::c_ulong,
    0xcee93d75 as libc::c_ulong,
    0xb2fe7a56b as libc::c_ulong,
    0xa847ed390 as libc::c_ulong,
    0x8713ef88c as libc::c_ulong,
    0xa217cc861 as libc::c_ulong,
    0x8bca25d7b as libc::c_ulong,
    0x455526818 as libc::c_ulong,
    0xea3a7a180 as libc::c_ulong,
    0xa9536e5e0 as libc::c_ulong,
    0x9b64a1975 as libc::c_ulong,
    0x5bfc756bc as libc::c_ulong,
    0x46aa169b as libc::c_ulong,
    0x53a17f76f as libc::c_ulong,
    0x4d6815274 as libc::c_ulong,
    0xcca9cf3f6 as libc::c_ulong,
    0x4013fcb8b as libc::c_ulong,
    0x3d26cdfa5 as libc::c_ulong,
    0x5786231f7 as libc::c_ulong,
    0x7d4ab09ab as libc::c_ulong,
    0x960b5ffbc as libc::c_ulong,
    0x8914df0d4 as libc::c_ulong,
    0x2fc6f2213 as libc::c_ulong,
    0xac235637e as libc::c_ulong,
    0x151b28ed3 as libc::c_ulong,
    0x46f79b6db as libc::c_ulong,
    0x1382e0c9f as libc::c_ulong,
    0x53abf983a as libc::c_ulong,
    0x383c47ade as libc::c_ulong,
    0x3fcf88978 as libc::c_ulong,
    0xeb9079df7 as libc::c_ulong,
    0x9af0714d as libc::c_ulong,
    0xda19d1bb7 as libc::c_ulong,
    0x9a02749f8 as libc::c_ulong,
    0x1c62dab9b as libc::c_ulong,
    0x1a137e44b as libc::c_ulong,
    0x2867718c7 as libc::c_ulong,
    0x35815525b as libc::c_ulong,
    0x7cd35c550 as libc::c_ulong,
    0x2164f73a0 as libc::c_ulong,
    0xe8b772fe0 as libc::c_ulong,
];
#[no_mangle]
pub unsafe extern "C" fn tag36h11_create() -> *mut apriltag_family_t {
    let mut tf: *mut apriltag_family_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<apriltag_family_t>() as libc::c_ulong,
    ) as *mut apriltag_family_t;
    let ref mut fresh0 = (*tf).name;
    *fresh0 = strdup(b"tag36h11\0" as *const u8 as *const libc::c_char);
    (*tf).h = 11 as libc::c_int as uint32_t;
    (*tf).ncodes = 587 as libc::c_int as uint32_t;
    let ref mut fresh1 = (*tf).codes;
    *fresh1 = codedata.as_mut_ptr();
    (*tf).nbits = 36 as libc::c_int as uint32_t;
    let ref mut fresh2 = (*tf).bit_x;
    *fresh2 = calloc(
        36 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    let ref mut fresh3 = (*tf).bit_y;
    *fresh3 = calloc(
        36 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    *((*tf).bit_x).offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(1 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(1 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(2 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(2 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(3 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(3 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(4 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(4 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(5 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(5 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(6 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(6 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(7 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(7 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(8 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(8 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(9 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(9 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(10 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(10 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(11 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(11 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(12 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(12 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(13 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(13 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(14 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(14 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(15 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(15 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(16 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(16 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(17 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(17 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(18 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(18 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(19 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(19 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(20 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(20 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(21 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(21 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(22 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(22 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(23 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(23 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(24 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(24 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(25 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(25 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(26 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(26 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(27 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(27 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(28 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(28 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(29 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(29 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(30 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(30 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(31 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(31 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(32 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(32 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(33 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(33 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(34 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(34 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(35 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(35 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    (*tf).width_at_border = 8 as libc::c_int;
    (*tf).total_width = 10 as libc::c_int;
    (*tf).reversed_border = 0 as libc::c_int != 0;
    return tf;
}
#[no_mangle]
pub unsafe extern "C" fn tag36h11_destroy(mut tf: *mut apriltag_family_t) {
    free((*tf).bit_x as *mut libc::c_void);
    free((*tf).bit_y as *mut libc::c_void);
    free((*tf).name as *mut libc::c_void);
    free(tf as *mut libc::c_void);
}
