use crate::tests::run_kernel_op;
use crate::PERIPHERALS;
use core::cell::Cell;
use kernel::hil::digest::{self, Digest, DigestData, DigestVerify, HMACSha256};
use kernel::static_init;
use kernel::utilities::cells::TakeCell;
use kernel::utilities::leasable_buffer::LeasableBuffer;
use kernel::{debug, ErrorCode};

static KEY: [u8; 32] = [0xA1; 32];

struct HmacTestCallback {
    add_data_done: Cell<bool>,
    verification_done: Cell<bool>,
    input_buffer: TakeCell<'static, [u8]>,
    digest_buffer: TakeCell<'static, [u8; 32]>,
}

unsafe impl Sync for HmacTestCallback {}

impl<'a> HmacTestCallback {
    fn new(input_buffer: &'static mut [u8], digest_buffer: &'static mut [u8; 32]) -> Self {
        HmacTestCallback {
            add_data_done: Cell::new(false),
            verification_done: Cell::new(false),
            input_buffer: TakeCell::new(input_buffer),
            digest_buffer: TakeCell::new(digest_buffer),
        }
    }

    fn reset(&self) {
        self.add_data_done.set(false);
        self.verification_done.set(false);
    }
}

impl<'a> digest::ClientData<'a, 32> for HmacTestCallback {
    fn add_data_done(&'a self, result: Result<(), ErrorCode>, data: &'static mut [u8]) {
        self.add_data_done.set(true);
        // Input data has been loaded, hold copy of data
        self.input_buffer.replace(data);
        assert_eq!(result, Ok(()));
    }
}

impl<'a> digest::ClientHash<'a, 32> for HmacTestCallback {
    fn hash_done(&'a self, _result: Result<(), ErrorCode>, _digest: &'static mut [u8; 32]) {
        unimplemented!()
    }
}

impl<'a> digest::ClientVerify<'a, 32> for HmacTestCallback {
    fn verification_done(
        &'a self,
        result: Result<bool, ErrorCode>,
        compare: &'static mut [u8; 32],
    ) {
        self.digest_buffer.replace(compare);
        self.verification_done.set(true);
        assert_eq!(result, Ok(true));
    }
}

/// Static init an HmacTestCallback, with
/// respective buffers allocated for data fields.
unsafe fn static_init_test_cb() -> &'static HmacTestCallback {
    let input_data = static_init!([u8; 32], [32; 32]);
    let digest_data = static_init!(
        [u8; 32],
        [
            0xdc, 0x55, 0x51, 0x5e, 0x30, 0xac, 0x50, 0xc7, 0x65, 0xbd, 0xe, 0x2, 0x82, 0xf7, 0x8b,
            0xe1, 0xef, 0xd1, 0xb, 0xdc, 0xa8, 0xba, 0xe1, 0xfa, 0x11, 0x3f, 0xf6, 0xeb, 0xaf,
            0x58, 0x57, 0x40,
        ]
    );

    static_init!(
        HmacTestCallback,
        HmacTestCallback::new(input_data, digest_data)
    )
}

#[test_case]
fn hmac_check_load_binary() {
    let perf = unsafe { PERIPHERALS.unwrap() };
    let hmac = &perf.hmac;

    let callback = unsafe { static_init_test_cb() };
    let buf = LeasableBuffer::new(callback.input_buffer.take().unwrap());

    debug!("check hmac load binary... ");
    run_kernel_op(100);

    hmac.set_client(callback);
    callback.reset();

    assert_eq!(hmac.add_data(buf), Ok(32));

    run_kernel_op(1000);
    #[cfg(feature = "hardware_tests")]
    assert_eq!(callback.add_data_done.get(), true);

    run_kernel_op(100);
    debug!("    [ok]");
    run_kernel_op(100);
}

#[test_case]
fn hmac_check_verify() {
    let perf = unsafe { PERIPHERALS.unwrap() };
    let hmac = &perf.hmac;

    let callback = unsafe { static_init_test_cb() };
    let buf = LeasableBuffer::new(callback.input_buffer.take().unwrap());

    debug!("check hmac check verify... ");
    run_kernel_op(100);

    hmac.set_client(callback);
    callback.reset();
    hmac.set_mode_hmacsha256(&KEY).unwrap();

    assert_eq!(hmac.add_data(buf), Ok(32));

    run_kernel_op(1000);
    #[cfg(feature = "hardware_tests")]
    assert_eq!(callback.add_data_done.get(), true);
    callback.reset();

    /* Get digest from callback digest buffer */
    assert_eq!(hmac.verify(callback.digest_buffer.take().unwrap()), Ok(()));

    run_kernel_op(1000);
    #[cfg(feature = "hardware_tests")]
    assert_eq!(callback.verification_done.get(), true);

    run_kernel_op(100);
    debug!("    [ok]");
    run_kernel_op(100);
}
