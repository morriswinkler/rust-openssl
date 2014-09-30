#![allow(non_camel_case_types)]
#![allow(dead_code)]
use libc::{c_void, c_int, c_char, c_ulong, c_long, c_uint, c_uchar};

pub type ASN1_INTEGER = c_void;
pub type ASN1_STRING = c_void;
pub type ASN1_TIME = c_void;
pub type BIO = c_void;
pub type BIO_METHOD = c_void;
pub type BN_CTX = *mut c_void;
pub type COMP_METHOD = c_void;
pub type CRYPTO_EX_DATA = c_void;
pub type ENGINE = c_void;
pub type EVP_CIPHER = *mut c_void;
pub type EVP_CIPHER_CTX = *mut c_void;
pub type EVP_MD = c_void;
pub type EVP_PKEY = *mut c_void;
pub type EVP_PKEY_CTX = c_void;
pub type RSA = *mut c_void;
pub type SSL = c_void;
pub type SSL_CTX = c_void;
pub type SSL_METHOD = c_void;
pub type X509 = c_void;
pub type X509_CRL = c_void;
pub type X509_EXTENSION = c_void;
pub type X509_NAME = c_void;
pub type X509_REQ = c_void;
pub type X509_STORE_CTX = c_void;

#[allow(dead_code)]
#[repr(C)]
pub struct BIGNUM {
    d: *mut c_void,
    top: c_int,
    dmax: c_int,
    pub neg: c_int,
    flags: c_int,
}

#[repr(C)]
pub struct EVP_MD_CTX {
    digest: *mut EVP_MD,
    engine: *mut c_void,
    flags: c_ulong,
    md_data: *mut c_void,
    pctx: *mut EVP_PKEY_CTX,
    update: *mut c_void
}

#[repr(C)]
pub struct HMAC_CTX {
    md: *mut EVP_MD,
    md_ctx: EVP_MD_CTX,
    i_ctx: EVP_MD_CTX,
    o_ctx: EVP_MD_CTX,
    key_length: c_uint,
    key: [c_uchar, ..128]
}

#[repr(C)]
pub struct X509V3_CTX {
    flags: c_int,
    issuer_cert: *mut c_void,
    subject_cert: *mut c_void,
    subject_req: *mut c_void,
    crl: *mut c_void,
    db_meth: *mut c_void,
    db: *mut c_void,
    // I like the last comment line, it is copied from OpenSSL sources:
    // Maybe more here
}

pub type CRYPTO_EX_new = extern "C" fn(parent: *mut c_void, ptr: *mut c_void,
                                       ad: *const CRYPTO_EX_DATA, idx: c_int,
                                       argl: c_long, argp: *const c_void) -> c_int;
pub type CRYPTO_EX_dup = extern "C" fn(to: *mut CRYPTO_EX_DATA,
                                       from: *mut CRYPTO_EX_DATA, from_d: *mut c_void,
                                       idx: c_int, argl: c_long, argp: *mut c_void)
                                       -> c_int;
pub type CRYPTO_EX_free = extern "C" fn(parent: *mut c_void, ptr: *mut c_void,
                                        ad: *mut CRYPTO_EX_DATA, idx: c_int,
                                        argl: c_long, argp: *mut c_void);
pub type PrivateKeyWriteCallback = extern "C" fn(buf: *mut c_char, size: c_int,
                                                 rwflag: c_int, user_data: *mut c_void)
                                                 -> c_int;


pub static CRYPTO_LOCK: c_int = 1;

pub static MBSTRING_ASC:  c_int = MBSTRING_FLAG | 1;
pub static MBSTRING_BMP:  c_int = MBSTRING_FLAG | 2;
pub static MBSTRING_FLAG: c_int = 0x1000;
pub static MBSTRING_UNIV: c_int = MBSTRING_FLAG | 4;
pub static MBSTRING_UTF8: c_int = MBSTRING_FLAG;

pub static NID_ext_key_usage: c_int = 126;
pub static NID_key_usage:     c_int = 83;

pub static SSL_CTRL_SET_TLSEXT_HOSTNAME: c_int = 55;
pub static SSL_ERROR_NONE: c_int = 0;
pub static SSL_ERROR_SSL: c_int = 1;
pub static SSL_ERROR_SYSCALL: c_int = 5;
pub static SSL_ERROR_WANT_ACCEPT: c_int = 8;
pub static SSL_ERROR_WANT_CONNECT: c_int = 7;
pub static SSL_ERROR_WANT_READ: c_int = 2;
pub static SSL_ERROR_WANT_WRITE: c_int = 3;
pub static SSL_ERROR_WANT_X509_LOOKUP: c_int = 4;
pub static SSL_ERROR_ZERO_RETURN: c_int = 6;
pub static SSL_VERIFY_NONE: c_int = 0;
pub static SSL_VERIFY_PEER: c_int = 1;

pub static TLSEXT_NAMETYPE_host_name: c_long = 0;

pub static V_ASN1_GENERALIZEDTIME: c_int = 24;
pub static V_ASN1_UTCTIME:         c_int = 23;

pub static X509_FILETYPE_ASN1: c_int = 2;
pub static X509_FILETYPE_DEFAULT: c_int = 3;
pub static X509_FILETYPE_PEM: c_int = 1;
pub static X509_V_ERR_AKID_ISSUER_SERIAL_MISMATCH: c_int = 31;
pub static X509_V_ERR_AKID_SKID_MISMATCH: c_int = 30;
pub static X509_V_ERR_APPLICATION_VERIFICATION: c_int = 50;
pub static X509_V_ERR_CERT_CHAIN_TOO_LONG: c_int = 22;
pub static X509_V_ERR_CERT_HAS_EXPIRED: c_int = 10;
pub static X509_V_ERR_CERT_NOT_YET_VALID: c_int = 9;
pub static X509_V_ERR_CERT_REJECTED: c_int = 28;
pub static X509_V_ERR_CERT_REVOKED: c_int = 23;
pub static X509_V_ERR_CERT_SIGNATURE_FAILURE: c_int = 7;
pub static X509_V_ERR_CERT_UNTRUSTED: c_int = 27;
pub static X509_V_ERR_CRL_HAS_EXPIRED: c_int = 12;
pub static X509_V_ERR_CRL_NOT_YET_VALID: c_int = 11;
pub static X509_V_ERR_CRL_PATH_VALIDATION_ERROR: c_int = 54;
pub static X509_V_ERR_CRL_SIGNATURE_FAILURE: c_int = 8;
pub static X509_V_ERR_DEPTH_ZERO_SELF_SIGNED_CERT: c_int = 18;
pub static X509_V_ERR_DIFFERENT_CRL_SCOPE: c_int = 44;
pub static X509_V_ERR_ERROR_IN_CERT_NOT_AFTER_FIELD: c_int = 14;
pub static X509_V_ERR_ERROR_IN_CERT_NOT_BEFORE_FIELD: c_int = 13;
pub static X509_V_ERR_ERROR_IN_CRL_LAST_UPDATE_FIELD: c_int = 15;
pub static X509_V_ERR_ERROR_IN_CRL_NEXT_UPDATE_FIELD: c_int = 16;
pub static X509_V_ERR_EXCLUDED_VIOLATION: c_int = 48;
pub static X509_V_ERR_INVALID_CA: c_int = 24;
pub static X509_V_ERR_INVALID_EXTENSION: c_int = 41;
pub static X509_V_ERR_INVALID_NON_CA: c_int = 37;
pub static X509_V_ERR_INVALID_POLICY_EXTENSION: c_int = 42;
pub static X509_V_ERR_INVALID_PURPOSE: c_int = 26;
pub static X509_V_ERR_KEYUSAGE_NO_CERTSIGN: c_int = 32;
pub static X509_V_ERR_KEYUSAGE_NO_CRL_SIGN: c_int = 35;
pub static X509_V_ERR_KEYUSAGE_NO_DIGITAL_SIGNATURE: c_int = 39;
pub static X509_V_ERR_NO_EXPLICIT_POLICY: c_int = 43;
pub static X509_V_ERR_OUT_OF_MEM: c_int = 17;
pub static X509_V_ERR_PATH_LENGTH_EXCEEDED: c_int = 25;
pub static X509_V_ERR_PERMITTED_VIOLATION: c_int = 47;
pub static X509_V_ERR_PROXY_CERTIFICATES_NOT_ALLOWED: c_int = 40;
pub static X509_V_ERR_PROXY_PATH_LENGTH_EXCEEDED: c_int = 38;
pub static X509_V_ERR_SELF_SIGNED_CERT_IN_CHAIN: c_int = 19;
pub static X509_V_ERR_SUBJECT_ISSUER_MISMATCH: c_int = 29;
pub static X509_V_ERR_SUBTREE_MINMAX: c_int = 49;
pub static X509_V_ERR_UNABLE_TO_DECODE_ISSUER_PUBLIC_KEY: c_int = 6;
pub static X509_V_ERR_UNABLE_TO_DECRYPT_CERT_SIGNATURE: c_int = 4;
pub static X509_V_ERR_UNABLE_TO_DECRYPT_CRL_SIGNATURE: c_int = 5;
pub static X509_V_ERR_UNABLE_TO_GET_CRL: c_int = 3;
pub static X509_V_ERR_UNABLE_TO_GET_CRL_ISSUER: c_int = 33;
pub static X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT: c_int = 2;
pub static X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT_LOCALLY: c_int = 20;
pub static X509_V_ERR_UNABLE_TO_VERIFY_LEAF_SIGNATURE: c_int = 21;
pub static X509_V_ERR_UNHANDLED_CRITICAL_CRL_EXTENSION: c_int = 36;
pub static X509_V_ERR_UNHANDLED_CRITICAL_EXTENSION: c_int = 34;
pub static X509_V_ERR_UNNESTED_RESOURCE: c_int = 46;
pub static X509_V_ERR_UNSUPPORTED_CONSTRAINT_SYNTAX: c_int = 52;
pub static X509_V_ERR_UNSUPPORTED_CONSTRAINT_TYPE: c_int = 51;
pub static X509_V_ERR_UNSUPPORTED_EXTENSION_FEATURE: c_int = 45;
pub static X509_V_ERR_UNSUPPORTED_NAME_SYNTAX: c_int = 53;
pub static X509_V_OK: c_int = 0;

#[cfg(target_os = "macos", feature = "tlsv1_1")]
#[cfg(target_os = "macos", feature = "tlsv1_2")]
#[link(name="ssl.1.0.0")]
#[link(name="crypto.1.0.0")]
extern {}

#[cfg(not(target_os = "macos"))]
#[cfg(target_os = "macos", not(feature = "tlsv1_1"), not(feature = "tlsv1_2"))]
#[link(name="ssl")]
#[link(name="crypto")]
extern {}

#[cfg(target_os = "win32")]
#[link(name="gdi32")]
#[link(name="wsock32")]
extern { }

extern "C" {
    pub fn ASN1_INTEGER_set(dest: *mut ASN1_INTEGER, value: c_long) -> c_int;
    pub fn ASN1_STRING_type_new(ty: c_int) -> *mut ASN1_STRING;

    pub fn BIO_free_all(a: *mut BIO);
    pub fn BIO_new(type_: *const BIO_METHOD) -> *mut BIO;
    pub fn BIO_read(b: *mut BIO, buf: *mut c_void, len: c_int) -> c_int;
    pub fn BIO_write(b: *mut BIO, buf: *const c_void, len: c_int) -> c_int;
    pub fn BIO_s_mem() -> *const BIO_METHOD;

    pub fn BN_new() -> *mut BIGNUM;
    pub fn BN_dup(n: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_clear_free(bn: *mut BIGNUM);

    pub fn BN_CTX_new() -> *mut BN_CTX;
    pub fn BN_CTX_free(ctx: *mut BN_CTX);

    pub fn BN_num_bits(bn: *mut BIGNUM) -> c_int;
    pub fn BN_set_negative(bn: *mut BIGNUM, n: c_int);
    pub fn BN_set_word(bn: *mut BIGNUM, n: c_ulong) -> c_int;

    /* Arithmetic operations on BIGNUMs */
    pub fn BN_add(r: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM) -> c_int;
    pub fn BN_div(dv: *mut BIGNUM, rem: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_exp(r: *mut BIGNUM, a: *mut BIGNUM, p: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_gcd(r: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_mod_add(r: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM, m: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_mod_exp(r: *mut BIGNUM, a: *mut BIGNUM, p: *mut BIGNUM, m: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_mod_inverse(r: *mut BIGNUM, a: *mut BIGNUM, n: *mut BIGNUM, ctx: *mut BN_CTX) -> *const BIGNUM;
    pub fn BN_mod_mul(r: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM, m: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_mod_sqr(r: *mut BIGNUM, a: *mut BIGNUM, m: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_mod_sub(r: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM, m: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_mul(r: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_nnmod(rem: *mut BIGNUM, a: *mut BIGNUM, m: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_sqr(r: *mut BIGNUM, a: *mut BIGNUM, ctx: *mut BN_CTX) -> c_int;
    pub fn BN_sub(r: *mut BIGNUM, a: *mut BIGNUM, b: *mut BIGNUM) -> c_int;

    /* Bit operations on BIGNUMs */
    pub fn BN_clear_bit(a: *mut BIGNUM, n: c_int) -> c_int;
    pub fn BN_is_bit_set(a: *mut BIGNUM, n: c_int) -> c_int;
    pub fn BN_lshift(r: *mut BIGNUM, a: *mut BIGNUM, n: c_int) -> c_int;
    pub fn BN_lshift1(r: *mut BIGNUM, a: *mut BIGNUM) -> c_int;
    pub fn BN_mask_bits(a: *mut BIGNUM, n: c_int) -> c_int;
    pub fn BN_rshift(r: *mut BIGNUM, a: *mut BIGNUM, n: c_int) -> c_int;
    pub fn BN_set_bit(a: *mut BIGNUM, n: c_int) -> c_int;
    pub fn BN_rshift1(r: *mut BIGNUM, a: *mut BIGNUM) -> c_int;

    /* Comparisons on BIGNUMs */
    pub fn BN_cmp(a: *mut BIGNUM, b: *mut BIGNUM) -> c_int;
    pub fn BN_ucmp(a: *mut BIGNUM, b: *mut BIGNUM) -> c_int;
    pub fn BN_is_zero(a: *mut BIGNUM) -> c_int;

    /* Prime handling */
    pub fn BN_generate_prime_ex(r: *mut BIGNUM, bits: c_int, safe: c_int, add: *mut BIGNUM, rem: *mut BIGNUM, cb: *const c_void) -> c_int;
    pub fn BN_is_prime_ex(p: *mut BIGNUM, checks: c_int, ctx: *mut BN_CTX, cb: *const c_void) -> c_int;
    pub fn BN_is_prime_fasttest_ex(p: *mut BIGNUM, checks: c_int, ctx: *mut BN_CTX, do_trial_division: c_int, cb: *const c_void) -> c_int;

    /* Random number handling */
    pub fn BN_rand(r: *mut BIGNUM, bits: c_int, top: c_int, bottom: c_int) -> c_int;
    pub fn BN_pseudo_rand(r: *mut BIGNUM, bits: c_int, top: c_int, bottom: c_int) -> c_int;
    pub fn BN_rand_range(r: *mut BIGNUM, range: *mut BIGNUM) -> c_int;
    pub fn BN_pseudo_rand_range(r: *mut BIGNUM, range: *mut BIGNUM) -> c_int;

    /* Conversion from/to binary representation */
    pub fn BN_bin2bn(s: *const u8, size: c_int, ret: *mut BIGNUM) -> *mut BIGNUM;
    pub fn BN_bn2bin(a: *mut BIGNUM, to: *mut u8) -> c_int;

    /* Conversion from/to string representation */
    pub fn BN_bn2dec(a: *mut BIGNUM) -> *const c_char;

    pub fn CRYPTO_num_locks() -> c_int;
    pub fn CRYPTO_set_locking_callback(func: extern "C" fn(mode: c_int,
                                                           n: c_int,
                                                           file: *const c_char,
                                                           line: c_int));
    pub fn CRYPTO_free(buf: *const c_char);

    pub fn ERR_get_error() -> c_ulong;

    pub fn EVP_md5() -> *const EVP_MD;
    pub fn EVP_ripemd160() -> *const EVP_MD;
    pub fn EVP_sha1() -> *const EVP_MD;
    pub fn EVP_sha224() -> *const EVP_MD;
    pub fn EVP_sha256() -> *const EVP_MD;
    pub fn EVP_sha384() -> *const EVP_MD;
    pub fn EVP_sha512() -> *const EVP_MD;

    pub fn EVP_aes_128_cbc() -> EVP_CIPHER;
    pub fn EVP_aes_128_ecb() -> EVP_CIPHER;
    // fn EVP_aes_128_ctr() -> EVP_CIPHER;
    // fn EVP_aes_128_gcm() -> EVP_CIPHER;
    pub fn EVP_aes_256_cbc() -> EVP_CIPHER;
    pub fn EVP_aes_256_ecb() -> EVP_CIPHER;
    // fn EVP_aes_256_ctr() -> EVP_CIPHER;
    // fn EVP_aes_256_gcm() -> EVP_CIPHER;
    pub fn EVP_rc4() -> EVP_CIPHER;

    pub fn EVP_CIPHER_CTX_new() -> EVP_CIPHER_CTX;
    pub fn EVP_CIPHER_CTX_set_padding(ctx: EVP_CIPHER_CTX, padding: c_int);
    pub fn EVP_CIPHER_CTX_free(ctx: EVP_CIPHER_CTX);

    pub fn EVP_CipherInit(ctx: EVP_CIPHER_CTX, evp: EVP_CIPHER,
                          key: *const u8, iv: *const u8, mode: c_int);
    pub fn EVP_CipherUpdate(ctx: EVP_CIPHER_CTX, outbuf: *mut u8,
                            outlen: &mut c_uint, inbuf: *const u8, inlen: c_int);
    pub fn EVP_CipherFinal(ctx: EVP_CIPHER_CTX, res: *mut u8, len: &mut c_int);

    pub fn EVP_DigestInit(ctx: *mut EVP_MD_CTX, typ: *const EVP_MD);
    pub fn EVP_DigestUpdate(ctx: *mut EVP_MD_CTX, data: *const u8, n: c_uint);
    pub fn EVP_DigestFinal(ctx: *mut EVP_MD_CTX, res: *mut u8, n: *mut u32);

    pub fn EVP_MD_CTX_create() -> *mut EVP_MD_CTX;
    pub fn EVP_MD_CTX_destroy(ctx: *mut EVP_MD_CTX);

    pub fn EVP_PKEY_new() -> *mut EVP_PKEY;
    pub fn EVP_PKEY_free(k: *mut EVP_PKEY);
    pub fn EVP_PKEY_assign(pkey: *mut EVP_PKEY, typ: c_int, key: *const c_char) -> c_int;
    pub fn EVP_PKEY_get1_RSA(k: *mut EVP_PKEY) -> *mut RSA;
    pub fn EVP_PKEY_set1_RSA(k: *mut EVP_PKEY, r: *mut RSA) -> c_int;

    pub fn HMAC_CTX_init(ctx: *mut HMAC_CTX);
    pub fn HMAC_Init_ex(ctx: *mut HMAC_CTX, key: *const u8, keylen: c_int, md: *const EVP_MD, imple: *const ENGINE);
    pub fn HMAC_Final(ctx: *mut HMAC_CTX, output: *mut u8, len: *mut c_uint);
    pub fn HMAC_Update(ctx: *mut HMAC_CTX, input: *const u8, len: c_uint);

    pub fn PEM_write_bio_PrivateKey(bio: *mut BIO, pkey: *mut EVP_PKEY, cipher: *const EVP_CIPHER,
                                    kstr: *mut c_char, klen: c_int,
                                    callback: *mut c_void,
                                    user_data: *mut c_void) -> c_int;
    pub fn PEM_write_bio_X509(bio: *mut BIO, x509: *mut X509) -> c_int;

    pub fn PKCS5_PBKDF2_HMAC_SHA1(pass: *const u8, passlen: c_int,
                                  salt: *const u8, saltlen: c_int,
                                  iter: c_int, keylen: c_int,
                                  out: *mut u8) -> c_int;


    pub fn RAND_bytes(buf: *mut u8, num: c_int) -> c_int;

    pub fn RSA_generate_key(modsz: c_uint, e: c_uint, cb: *const u8, cbarg: *const u8) -> *mut RSA;
    pub fn RSA_private_decrypt(flen: c_uint, from: *const u8, to: *mut u8, k: *mut RSA,
                               pad: c_int) -> c_int;
    pub fn RSA_public_encrypt(flen: c_uint, from: *const u8, to: *mut u8, k: *mut RSA,
                              pad: c_int) -> c_int;
    pub fn RSA_sign(t: c_int, m: *const u8, mlen: c_uint, sig: *mut u8, siglen: *mut c_uint,
                    k: *mut RSA) -> c_int;
    pub fn RSA_size(k: *mut RSA) -> c_uint;
    pub fn RSA_verify(t: c_int, m: *const u8, mlen: c_uint, sig: *const u8, siglen: c_uint,
                      k: *mut RSA) -> c_int;

    pub fn SSL_library_init() -> c_int;

    #[cfg(feature = "sslv2")]
    pub fn SSLv2_method() -> *const SSL_METHOD;
    pub fn SSLv3_method() -> *const SSL_METHOD;
    pub fn TLSv1_method() -> *const SSL_METHOD;
    #[cfg(feature = "tlsv1_1")]
    pub fn TLSv1_1_method() -> *const SSL_METHOD;
    #[cfg(feature = "tlsv1_2")]
    pub fn TLSv1_2_method() -> *const SSL_METHOD;
    pub fn SSLv23_method() -> *const SSL_METHOD;

    pub fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    pub fn SSL_free(ssl: *mut SSL);
    pub fn SSL_set_bio(ssl: *mut SSL, rbio: *mut BIO, wbio: *mut BIO);
    pub fn SSL_get_rbio(ssl: *mut SSL) -> *mut BIO;
    pub fn SSL_get_wbio(ssl: *mut SSL) -> *mut BIO;
    pub fn SSL_connect(ssl: *mut SSL) -> c_int;
    pub fn SSL_ctrl(ssl: *mut SSL, cmd: c_int, larg: c_long,
                    parg: *mut c_void) -> c_long;
    pub fn SSL_get_error(ssl: *mut SSL, ret: c_int) -> c_int;
    pub fn SSL_read(ssl: *mut SSL, buf: *mut c_void, num: c_int) -> c_int;
    pub fn SSL_write(ssl: *mut SSL, buf: *const c_void, num: c_int) -> c_int;
    pub fn SSL_get_ex_data_X509_STORE_CTX_idx() -> c_int;
    pub fn SSL_get_SSL_CTX(ssl: *mut SSL) -> *mut SSL_CTX;
    pub fn SSL_get_current_compression(ssl: *mut SSL) -> *const COMP_METHOD;

    pub fn SSL_COMP_get_name(comp: *const COMP_METHOD) -> *const c_char;

    pub fn SSL_CTX_new(method: *const SSL_METHOD) -> *mut SSL_CTX;
    pub fn SSL_CTX_free(ctx: *mut SSL_CTX);
    pub fn SSL_CTX_set_verify(ctx: *mut SSL_CTX, mode: c_int,
                              verify_callback: Option<extern fn(c_int, *mut X509_STORE_CTX) -> c_int>);
    pub fn SSL_CTX_load_verify_locations(ctx: *mut SSL_CTX, CAfile: *const c_char,
                                         CApath: *const c_char) -> c_int;
    pub fn SSL_CTX_get_ex_new_index(argl: c_long, argp: *const c_void,
                                    new_func: Option<CRYPTO_EX_new>,
                                    dup_func: Option<CRYPTO_EX_dup>,
                                    free_func: Option<CRYPTO_EX_free>)
                                    -> c_int;
    pub fn SSL_CTX_set_ex_data(ctx: *mut SSL_CTX, idx: c_int, data: *mut c_void)
                               -> c_int;
    pub fn SSL_CTX_get_ex_data(ctx: *mut SSL_CTX, idx: c_int) -> *mut c_void;

    pub fn SSL_CTX_use_certificate_file(ctx: *mut SSL_CTX, cert_file: *const c_char, file_type: c_int) -> c_int;
    pub fn SSL_CTX_use_PrivateKey_file(ctx: *mut SSL_CTX, key_file: *const c_char, file_type: c_int) -> c_int;


    pub fn X509_add_ext(x: *mut X509, ext: *mut X509_EXTENSION, loc: c_int) -> c_int;
    pub fn X509_digest(x: *mut X509, digest: *const EVP_MD, buf: *mut c_char, len: *mut c_uint) -> c_int;
    pub fn X509_get_serialNumber(x: *mut X509) -> *mut ASN1_INTEGER;
    pub fn X509_get_subject_name(x: *mut X509) -> *mut X509_NAME;
    pub fn X509_gmtime_adj(time: *mut ASN1_TIME, adj: c_long) -> *mut ASN1_TIME;
    pub fn X509_new() -> *mut X509;
    pub fn X509_set_issuer_name(x: *mut X509, name: *mut X509_NAME) -> c_int;
    pub fn X509_set_notAfter(x: *mut X509, tm: *const ASN1_TIME) -> c_int;
    pub fn X509_set_notBefore(x: *mut X509, tm: *const ASN1_TIME) -> c_int;
    pub fn X509_set_version(x: *mut X509, version: c_ulong) -> c_int;
    pub fn X509_set_pubkey(x: *mut X509, pkey: *mut EVP_PKEY) -> c_int;
    pub fn X509_sign(x: *mut X509, pkey: *mut EVP_PKEY, md: *const EVP_MD) -> c_int;

    pub fn X509_EXTENSION_free(ext: *mut X509_EXTENSION);

    pub fn X509_NAME_add_entry_by_txt(x: *mut X509, field: *const c_char, ty: c_int, bytes: *const c_char, len: c_int, loc: c_int, set: c_int) -> c_int;

    pub fn X509_STORE_CTX_get_current_cert(ct: *mut X509_STORE_CTX) -> *mut X509;
    pub fn X509_STORE_CTX_get_error(ctx: *mut X509_STORE_CTX) -> c_int;
    pub fn X509_STORE_CTX_get_ex_data(ctx: *mut X509_STORE_CTX, idx: c_int) -> *mut c_void;

    pub fn X509V3_EXT_conf_nid(conf: *mut c_void, ctx: *mut X509V3_CTX, ext_nid: c_int, value: *mut c_char) -> *mut X509_EXTENSION;
    pub fn X509V3_set_ctx(ctx: *mut X509V3_CTX, issuer: *mut X509, subject: *mut X509, req: *mut X509_REQ, crl: *mut X509_CRL, flags: c_int);

    pub fn i2d_RSA_PUBKEY(k: *mut RSA, buf: *const *mut u8) -> c_int;
    pub fn d2i_RSA_PUBKEY(k: *const *mut RSA, buf: *const *const u8, len: c_uint) -> *mut RSA;
    pub fn i2d_RSAPrivateKey(k: *mut RSA, buf: *const *mut u8) -> c_int;
    pub fn d2i_RSAPrivateKey(k: *const *mut RSA, buf: *const *const u8, len: c_uint) -> *mut RSA;
}