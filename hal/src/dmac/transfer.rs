//! # DMA transfer abstractions
//!
//! # Transfer types
//!
//! Four basic transfer types are supported:
//!
//! * Incrementing-source to incrementing-destination (normally used for
//!   memory-to-memory transfers)
//!
//! * Incrementing-source to fixed-destination (normally used for
//!   memory-to-peripheral transfers)
//!
//! * Fixed-source to incrementing-destination (normally used for
//!   peripheral-to-memory transfers)
//!
//! * Fixed-source to fixed-destination (normally used for
//!   peripheral-to-peripheral transfers)
//!
//! # Beat sizes
//!
//! A beat is an atomic, uninterruptible transfer size.Three beat sizes are
//! supported:
//!
//! * Byte-per-byte (8 bit beats);
//!
//! * Halfword-per-halfword (16 bit beats);
//!
//! * Word-per-word (32 bit beats);
//!
//! The correct beat size will automatically be selected in function of the type
//! of the source and destination buffers.
//!
//! # One-shot vs circular transfers
//!
//! If the transfer is setup as one-shot (`circular == false`), the
//! transfer will run once and stop. Otherwise, if `circular == true`, then the
//! transfer will be set as circular, meaning it will restart a new, identical
//! block transfer when the current block transfer is complete. This is
//! particularly useful when combined with a TC/TCC trigger source, for instance
//! to periodically retreive a sample from an ADC and send it to a circular
//! buffer, or send a sample to a DAC.
//!
//! # Starting a transfer
//!
//! A transfer is started by calling [`Transfer::begin`]. You will be
//! required to supply a trigger source and a trigger action.
//!
//! # Waiting for a transfer to complete
//!
//! A transfer can waited upon by calling [`wait`](Transfer::wait). This is a
//! _blocking_ method, meaning it will busy-wait until the transfer is
//! completed. When it returns, it will release the source and destination
//! buffers, as well as the DMA channel.
//!
//! # Interrupting (stopping) a transfer
//!
//! A transfer can be stopped (regardless of whether it has completed or not) by
//! calling [`stop`](Transfer::stop). This is _not_ a blocking method,
//! meaning it will stop the transfer and immediately return. When it returns,
//! it will release the source and destination buffers, as well as the DMA
//! channel.
//!
//! # Trigger sources
//!
//! Most peripherals can issue triggers to a DMA channel. A software trigger is
//! also available (see [`trigger`](Transfer::software_trigger)). See
//! ATSAMD21 datasheet, table 19-8 for all available trigger sources.
//!
//! # Trigger actions
//!
//! Three trigger actions are available:
//!
//! * BLOCK: One trigger required for each block transfer. In the context of
//!   this driver, one Transfer is equivalent to one Block transfer.
//!
//! * BEAT: One trigger required for each beat transfer. In the context of this
//!   driver, the beat size will depend on the type of buffer used (8, 16 or 32
//!   bits).
//!
//! * TRANSACTION: One trigger required for a full DMA transaction. this is
//!   useful for circular transfers in the context of this driver. One trigger
//!   will set off the transaction, that will now run uninterrupted until it is
//!   stopped.

use super::{
    Error, Result,
    channel::{AnyChannel, Busy as ChBusy, Channel, InterruptFlags, StatusFlags, Ready as ChReady},
    dma_controller::{TriggerAction, TriggerSource, ChId},
};
use crate::typelevel::{Is, Sealed};
use modular_bitfield::prelude::*;
use voltserver_hal::dma::Buffer as DmaBuffer;

//==============================================================================
// Beat
//==============================================================================

/// Useable beat sizes for DMA transfers
#[derive(Clone, Copy, BitfieldSpecifier)]
#[bits = 2]
pub enum BeatSize {
    /// Byte = [`u8`](core::u8)
    Byte = 0x00,
    /// Half word = [`u16`](core::u16)
    HalfWord = 0x01,
    /// Word = [`u32`](core::u32)
    Word = 0x02,
}

/// Convert 8, 16 and 32 bit types
/// into [`BeatSize`]
///
/// # Safety
///
/// This trait should not be implemented outside of the crate-provided
/// implementations
pub unsafe trait Beat: Sealed + voltserver_hal::dma::Word {
    /// Convert to BeatSize enum
    const BEATSIZE: BeatSize;
}

macro_rules! impl_beat {
    ( $( ($Type:ty, $Size:ident) ),+ ) => {
        $(
            unsafe impl Beat for $Type {
                const BEATSIZE: BeatSize = BeatSize::$Size;
            }
        )+
    };
}

impl_beat!(
    (u8, Byte),
    (i8, Byte),
    (u16, HalfWord),
    (i16, HalfWord),
    (u32, Word),
    (i32, Word),
    (f32, Word)
);

//==============================================================================
// Buffer
//==============================================================================

/// Buffer useable by the DMAC.
///
/// # Safety
///
/// This trait should only be implemented for valid DMAC sources/sinks. That is,
/// you need to make sure that:
/// * `dma_ptr` points to a valid memory location useable by the DMAC
/// * `incrementing` is correct for the source/sink. For example, an `&[u8]` of
///   size one is not incrementing.
/// * `buffer_len` is correct for the source/sink.
pub unsafe trait Buffer: DmaBuffer<Self::Beat> {
    /// DMAC beat size
    type Beat: Beat;
    /// Pointer to the buffer. If the buffer is incrementing, the address should
    /// point to one past the last beat transfer in the block.
    fn dma_ptr(&mut self) -> *mut Self::Beat;
    /// Return whether the buffer pointer should be incrementing or not
    fn incrementing(&self) -> bool;
    /// Buffer length in beats
    fn buffer_len(&self) -> usize;
}

pub trait PeripheralBuffer: Buffer {
    const TRIG_SRC: TriggerSource;
}

unsafe impl<T: Beat, const N: usize> Buffer for &'static mut [T; N] {
    type Beat = T;
    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        let ptrs = self.as_mut_ptr_range();
        if self.incrementing() {
            ptrs.end
        } else {
            ptrs.start
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        N > 1
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        N
    }
}
//
//unsafe impl<T: Beat> Buffer for &mut [T] {
//    type Beat = T;
//    #[inline]
//    fn dma_ptr(&mut self) -> *mut Self::Beat {
//        let ptrs = self.as_mut_ptr_range();
//        if self.incrementing() {
//            ptrs.end
//        } else {
//            ptrs.start
//        }
//    }
//
//    #[inline]
//    fn incrementing(&self) -> bool {
//        self.len() > 1
//    }
//
//    #[inline]
//    fn buffer_len(&self) -> usize {
//        self.len()
//    }
//}
//
//unsafe impl<T: Beat> Buffer for &mut T {
//    type Beat = T;
//    #[inline]
//    fn dma_ptr(&mut self) -> *mut Self::Beat {
//        *self as *mut T
//    }
//
//    #[inline]
//    fn incrementing(&self) -> bool {
//        false
//    }
//
//    #[inline]
//    fn buffer_len(&self) -> usize {
//        1
//    }
//}

//==============================================================================
// BufferPair
//==============================================================================

/// Struct holding the source and destination buffers of a
/// [`Transfer`].
pub struct BufferPair<S, D = S>
where
    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
{
    /// Source buffer
    pub source: S,
    /// Destination buffer
    pub destination: D,
}

//==============================================================================
// AnyBufferPair
//==============================================================================

pub trait AnyBufferPair: Sealed + Is<Type = SpecificBufferPair<Self>> {
    type Src: Buffer + voltserver_hal::dma::SrcBuffer<BufferPairBeat<Self>>;
    type Dst: Buffer<Beat = BufferPairBeat<Self>> + voltserver_hal::dma::DstBuffer<BufferPairBeat<Self>>;

    fn source(&mut self) -> &mut Self::Src;
    fn destination(&mut self) -> &mut Self::Dst;
}

pub type SpecificBufferPair<C> = BufferPair<<C as AnyBufferPair>::Src, <C as AnyBufferPair>::Dst>;

pub type BufferPairSrc<B> = <B as AnyBufferPair>::Src;
pub type BufferPairDst<B> = <B as AnyBufferPair>::Dst;
pub type BufferPairBeat<B> = <BufferPairSrc<B> as Buffer>::Beat;

impl<S, D> Sealed for BufferPair<S, D>
where
    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
{
}

impl<S, D> AnyBufferPair for BufferPair<S, D>
where
    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
{
    type Src = S;
    type Dst = D;

    fn source(&mut self) -> &mut Self::Src {
        &mut self.source
    }

    fn destination(&mut self) -> &mut Self::Dst {
        &mut self.destination
    }
}

impl<S, D> AsRef<Self> for BufferPair<S, D>
where
    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<S, D> AsMut<Self> for BufferPair<S, D>
where
    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// Transfer State
//==============================================================================
pub trait State: Sealed {
    type Chan: AnyChannel;

    fn channel(&self) -> &Self::Chan;
    fn channel_mut(&mut self) -> &mut Self::Chan;
}

type StateChannelId<St> = <<St as State>::Chan as AnyChannel>::Id;

pub struct Ready<Id: ChId> (pub Channel<Id, ChReady>);
pub struct Busy<Id: ChId> (pub Channel<Id, ChBusy>);
pub struct Complete<Id: ChId> (pub Channel<Id, ChReady>);

impl<Id: ChId> Sealed for Ready<Id> {}
impl<Id: ChId> State for Ready<Id> {
    type Chan = Channel<Id, ChReady>;

    fn channel(&self) -> &Self::Chan {
        &self.0
    }

    fn channel_mut(&mut self) -> &mut Self::Chan {
        &mut self.0
    }
}
impl<Id: ChId> Sealed for Busy<Id> {}
impl<Id: ChId> State for Busy<Id> {
    type Chan = Channel<Id, ChBusy>;

    fn channel(&self) -> &Self::Chan {
        &self.0
    }

    fn channel_mut(&mut self) -> &mut Self::Chan {
        &mut self.0
    }
}
impl<Id: ChId> Sealed for Complete<Id> {}
impl<Id: ChId> State for Complete<Id> {
    type Chan = Channel<Id, ChReady>;

    fn channel(&self) -> &Self::Chan {
        &self.0
    }

    fn channel_mut(&mut self) -> &mut Self::Chan {
        &mut self.0
    }
}

//==============================================================================
// AnyTransfer
//==============================================================================
pub trait AnyTransfer: Sealed + Is<Type = SpecificTransfer<Self>> {
    type Buf: AnyBufferPair;
    type State: State;

    /// Check if the channel has any error flags set. Returns `Ok` if no error
    /// flags are set, otherwise returns the flags.
    fn channel_error(&mut self) -> core::result::Result<(), StatusFlags>;
    /// Clear channel error flags
    fn clear_channel_errors(&mut self);
}

pub type SpecificTransfer<T> = Transfer<<T as AnyTransfer>::Buf, <T as AnyTransfer>::State>;

pub type TransferState<T> = <T as AnyTransfer>::State;
pub type TransferChannel<T> = <<T as AnyTransfer>::State as State>::Chan;
pub type TransferChannelId<T> = <<<T as AnyTransfer>::State as State>::Chan as AnyChannel>::Id;
pub type TransferBuffers<T> = <T as AnyTransfer>::Buf;
pub type TransferSourceBuffer<T> = <<T as AnyTransfer>::Buf as AnyBufferPair>::Src;
pub type TransferDestinationBuffer<T> = <<T as AnyTransfer>::Buf as AnyBufferPair>::Dst;

impl<Buf, S> Sealed for Transfer<Buf, S>
where
    Buf: AnyBufferPair,
    S: State,
{
}

impl<Buf, S> AnyTransfer for Transfer<Buf, S>
where
    Buf: AnyBufferPair,
    S: State
{
    type Buf = Buf;
    type State = S;

    fn channel_error(&mut self) -> core::result::Result<(), StatusFlags> {
        self.state.channel_mut().channel_error()
    }

    fn clear_channel_errors(&mut self) {
        self.state.channel_mut().clear_channel_errors();
    }
}

impl<Buf, S> AsRef<Self> for Transfer<Buf, S>
where
    Buf: AnyBufferPair,
    S: State,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Buf, S> AsMut<Self> for Transfer<Buf, S>
where
    Buf: AnyBufferPair,
    S: State,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// ReadyTransfer
//==============================================================================
pub trait ReadyTransfer: AnyTransfer {
    type Busy: BusyTransfer<Buf = TransferBuffers<Self>>;

    fn begin(self) -> Self::Busy;
    fn free(self) -> (TransferChannel<Self>, TransferSourceBuffer<Self>, TransferDestinationBuffer<Self>);
}
impl<Buf, Id> ReadyTransfer for Transfer<Buf, Ready<Id>>
where
    Buf: AnyBufferPair,
    Id: ChId,
{
    type Busy = Transfer<Buf, Busy<Id>>;

    #[inline]
    fn begin(self) -> Self::Busy {
        self.begin()
    }

    #[inline]
    fn free(self) -> (TransferChannel<Self>, TransferSourceBuffer<Self>, TransferDestinationBuffer<Self>) {
        self.free()
    }
}

//==============================================================================
// BusyTransfer
//==============================================================================
pub trait BusyTransfer: AnyTransfer {
    type Complete: CompleteTransfer<Buf = TransferBuffers<Self>>;

    fn software_trigger(&mut self);
    unsafe fn borrow_source(&mut self) -> &mut TransferSourceBuffer<Self>;
    unsafe fn borrow_destination(&mut self) -> &mut TransferDestinationBuffer<Self>;
    fn is_complete(&mut self) -> bool;
    fn wait(self) -> Self::Complete;
    fn stop(self) -> Self::Complete;

}
impl<Buf, Id> BusyTransfer for Transfer<Buf, Busy<Id>>
where
    Buf: AnyBufferPair,
    Id: ChId,
{
    type Complete = Transfer<Buf, Complete<Id>>;

    #[inline]
    fn software_trigger(&mut self) {
        self.software_trigger();
    }

    #[inline]
    unsafe fn borrow_source(&mut self) -> &mut TransferSourceBuffer<Self> {
        unsafe { self.borrow_source() }
    }

    #[inline]
    unsafe fn borrow_destination(&mut self) -> &mut TransferDestinationBuffer<Self> {
        unsafe { self.borrow_destination() }
    }

    #[inline]
    fn is_complete(&mut self) -> bool {
        self.is_complete()
    }

    #[inline]
    fn wait(self) -> Self::Complete {
        self.wait()
    }

    #[inline]
    fn stop(self) -> Self::Complete {
        self.stop()
    }
}

//==============================================================================
// CompleteTransfer
//==============================================================================
pub trait CompleteTransfer: AnyTransfer {
    type Ready: ReadyTransfer<Buf = TransferBuffers<Self>>;
    type Busy: BusyTransfer<Buf = TransferBuffers<Self>>;

    fn borrow_source(&mut self) -> &mut TransferSourceBuffer<Self>;
    fn borrow_destination(&mut self) -> &mut TransferDestinationBuffer<Self>;
    fn recycle(self, source: TransferSourceBuffer<Self>, destination: TransferDestinationBuffer<Self>) -> Result<(Self::Busy, TransferSourceBuffer<Self>, TransferDestinationBuffer<Self>)>;
    fn recycle_source(self, destination: TransferDestinationBuffer<Self>) -> Result<(Self::Busy, TransferDestinationBuffer<Self>)>;
    fn recycle_destination(self, source: TransferSourceBuffer<Self>) -> Result<(Self::Busy, TransferSourceBuffer<Self>)>;
    fn reset(self) -> Self::Ready;
    fn free(self) -> (TransferChannel<Self>, TransferSourceBuffer<Self>, TransferDestinationBuffer<Self>);
}
impl<Buf, Id> CompleteTransfer for Transfer<Buf, Complete<Id>>
where
    Buf: AnyBufferPair,
    Id: ChId,
{
    type Ready = Transfer<Buf, Ready<Id>>;
    type Busy = Transfer<Buf, Busy<Id>>;

    #[inline]
    fn borrow_source(&mut self) -> &mut TransferSourceBuffer<Self> {
        self.borrow_source()
    }

    #[inline]
    fn borrow_destination(&mut self) -> &mut TransferDestinationBuffer<Self> {
        self.borrow_destination()
    }

    #[inline]
    fn recycle(self, source: TransferSourceBuffer<Self>, destination: TransferDestinationBuffer<Self>) -> Result<(Self::Busy, TransferSourceBuffer<Self>, TransferDestinationBuffer<Self>)> {
        self.recycle(source, destination)
    }

    #[inline]
    fn recycle_source(self, destination: TransferDestinationBuffer<Self>) -> Result<(Self::Busy, TransferDestinationBuffer<Self>)> {
        self.recycle_source(destination)
    }

    #[inline]
    fn recycle_destination(self, source: TransferSourceBuffer<Self>) -> Result<(Self::Busy, TransferSourceBuffer<Self>)> {
        self.recycle_destination(source)
    }

    #[inline]
    fn reset(self) -> Self::Ready {
        self.reset()
    }

    #[inline]
    fn free(self) -> (TransferChannel<Self>, TransferSourceBuffer<Self>, TransferDestinationBuffer<Self>) {
        self.free()
    }
}

// TODO change source and dest types to Pin? (see https://docs.rust-embedded.org/embedonomicon/dma.html#immovable-buffers)
/// DMA transfer, owning the resources until the transfer is done and
/// [`Transfer::wait`] is called.
pub struct Transfer<Buf, St>
where
    Buf: AnyBufferPair,
    St: State,
{
    state: St,
    buffers: Buf,
    complete: bool,
    trig_src: TriggerSource,
    trig_act: TriggerAction,
}

impl<B, Id> Transfer<B, Ready<Id>>
where
    B: AnyBufferPair,
    Id: ChId,
{
    /// Safely construct a new `Transfer`. To guarantee memory safety, both
    /// buffers are required to be `'static`.
    /// Refer [here](https://docs.rust-embedded.org/embedonomicon/dma.html#memforget) or
    /// [here](https://blog.japaric.io/safe-dma/) for more information.
    ///
    /// If two array references can be used as source and destination buffers
    /// (as opposed to slices), then it is recommended to use the
    /// [`Transfer::new_from_arrays`] method instead.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::LengthMismatch`] if both buffers have a length > 1
    ///   and are not of equal length.
    /// * Returns [`Error::TooManyBeats`] if the number of beats are greater
    ///   than `u16::MAX``.
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new(
        chan: <Ready<Id> as State>::Chan,
        source: BufferPairSrc<B>,
        destination: BufferPairDst<B>,
        circular: bool,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Result<Transfer<B, Ready<Id>>> {
        Self::check_buffer_pair(&source, &destination)?;

        // SAFETY: The safety checks are done by the function signature and the buffer
        // length verification
        Ok(unsafe { Self::new_unchecked(chan, source, destination, circular, trig_src, trig_act) })
    }
}

impl<B, St> Transfer<B, St>
where
    B: AnyBufferPair,
    St: State,
{
    #[inline]
    pub(super) fn check_buffer_pair(source: &BufferPairSrc<B>, destination: &BufferPairDst<B>) -> Result<()> {
        let src_len = source.buffer_len();
        let dst_len = destination.buffer_len();

        if src_len > 1 && dst_len > 1 && src_len != dst_len {
            Err(Error::LengthMismatch)
        } else if src_len.max(dst_len) > u16::MAX.into() {
            Err(Error::TooManyBeats)
        } else {
            Ok(())
        }
    }

}

impl<B, Id> Transfer<B, Ready<Id>>
where
    B: AnyBufferPair,
    Id: ChId,
{
    /// Construct a new `Transfer` without checking for memory safety.
    ///
    /// # Safety
    ///
    /// To guarantee the safety of creating a `Transfer` using this method, you
    /// must uphold some invariants:
    ///
    /// * A `Transfer` holding a `Channel<Id, Running>` must *never* be dropped.
    ///   It should *always* be explicitly be `wait`ed upon or `stop`ped.
    ///
    /// * The size in bytes or the source and destination buffers should be
    ///   exacly the same, unless one or both buffers are of length 1. The
    ///   transfer length will be set to the longest of both buffers if they are
    ///   not of equal size.
    ///
    /// * The source and destination buffers should have a length smaller or
    ///   equal to `u16::MAX`.
    #[inline]
    pub unsafe fn new_unchecked(
        mut chan: <Ready<Id> as State>::Chan,
        mut source: BufferPairSrc<B>,
        mut destination: BufferPairDst<B>,
        circular: bool,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Transfer<B, Ready<Id>> {
        unsafe {
            chan.as_mut()
                .fill_descriptor(&mut source, &mut destination, circular);
        }

        let buffers = BufferPair {
            source,
            destination,
        };

        Transfer {
            state: Ready(chan),
            buffers: buffers.into(),
            complete: false,
            trig_src,
            trig_act,
        }
    }
}

impl<B, Id> Transfer<B, Ready<Id>>
where
    B: AnyBufferPair,
    Id: ChId,
{
    /// Begin DMA transfer in blocking mode. If [`TriggerSource::Disable`] is
    /// used, a software trigger will be issued to the DMA channel to launch
    /// the transfer. Is is therefore not necessary, in most cases, to manually
    /// issue a software trigger to the channel.
    #[inline]
    pub fn begin(
        mut self,
    ) -> Transfer<B, Busy<Id>> {
        // Reset the complete flag before triggering the transfer.
        // This way an interrupt handler could set complete to true
        // before this function returns.
        self.complete = false;

        let chan = self.state.0.start(self.trig_src, self.trig_act);

        Transfer {
            state: Busy(chan),
            buffers: self.buffers,
            complete: self.complete,
            trig_src: self.trig_src,
            trig_act: self.trig_act,
        }
    }

    /// Free the [`Transfer`] and return the resources it holds.
    ///
    /// Similar to [`stop`](Transfer::stop), but it acts on a [`Transfer`]
    /// holding a [`Ready`] channel, so there is no need to explicitly stop the
    /// transfer.
    pub fn free(self) -> (Channel<Id, ChReady>, BufferPairSrc<B>, BufferPairDst<B>) {
        let buffers: SpecificBufferPair<B> = self.buffers.into();
        (
            self.state.0.into(),
            buffers.source,
            buffers.destination,
        )
    }
}

//impl<B, const N: usize, Id> Transfer<BufferPair<&'static mut [B; N]>, Ready<Id>>
//where
//    B: 'static + Beat,
//    Id: ChId,
//{
//    /// Create a new `Transfer` from static array references of the same type
//    /// and length. When two array references are available (instead of slice
//    /// references), it is recommended to use this function over
//    /// [`Transfer::new`](Transfer::new), because it provides compile-time
//    /// checking that the array lengths match. It therefore does not panic, and
//    /// saves some runtime checking of the array lengths.
//    #[inline]
//    pub fn new_from_arrays(
//        chan: <Ready<Id> as State>::Chan,
//        source: &'static mut [B; N],
//        destination: &'static mut [B; N],
//        circular: bool,
//        trig_src: TriggerSource,
//        trig_act: TriggerAction,
//    ) -> Self {
//        unsafe { Self::new_unchecked(chan, source, destination, circular, trig_src, trig_act) }
//    }
//}

impl<B, Id> Transfer<B, Busy<Id>>
where
    B: AnyBufferPair,
    Id: ChId,
{
    /// Issue a software trigger request to the corresponding channel.
    /// Note that is not guaranteed that the trigger request will register,
    /// if a trigger request is already pending for the channel.
    #[inline]
    pub fn software_trigger(&mut self) {
        self.state.0.as_mut().software_trigger();
    }

    /// Unsafely and mutably borrow the source buffer
    ///
    /// # Safety
    ///
    /// The source buffer should never be borrowed when a transfer is in
    /// progress, as it is getting mutated or read in another context (ie,
    /// the DMAC hardware "thread").
    #[expect(dead_code)]
    #[inline]
    pub(crate) unsafe fn borrow_source(&mut self) -> &mut BufferPairSrc<B> {
        self.buffers.source()
    }

    /// Unsafely and mutably borrow the destination buffer.
    ///
    /// # Safety
    ///
    /// The destination buffer should never be borrowed when a transfer is in
    /// progress, as it is getting mutated or read in another context (ie,
    /// the DMAC hardware "thread").
    #[expect(dead_code)]
    #[inline]
    pub(crate) unsafe fn borrow_destination(&mut self) -> &mut BufferPairDst<B> {
        self.buffers.destination()
    }

    /// Wait for the DMA transfer to complete
    ///
    /// # Blocking: This method may block
    #[inline]
    pub fn wait(mut self) -> Transfer<B, Complete<Id>> {
        // Wait for transfer to complete
        while !self.is_complete() {}
        self.stop()
    }

    /// Check if the transfer has completed
    #[inline]
    pub fn is_complete(&mut self) -> bool {
        if !self.complete {
            let chan = self.state.0.as_mut();
            let complete = chan.xfer_complete();
            self.complete = complete;
        }
        self.complete
    }

    /// Checks and clears the block transfer complete interrupt flag
    #[inline]
    pub fn block_transfer_interrupt(&mut self) -> bool {
        self.state.0
            .as_mut()
            .check_and_clear_interrupts(InterruptFlags::new().with_tcmpl(true))
            .tcmpl()
    }

    /// Non-blocking; Immediately stop the DMA transfer
    #[inline]
    pub fn stop(self) -> Transfer<B, Complete<Id>> {
        // `free()` stops the transfer, waits for the burst to finish, and emits a
        // compiler fence.
        let chan = self.state.0.free();

        Transfer {
            state: Complete(chan),
            buffers: self.buffers,
            complete: true,
            trig_src: self.trig_src,
            trig_act: self.trig_act,
        }
    }
}

impl<B, Id> Transfer<B, Complete<Id>>
where
    B: AnyBufferPair,
    Id: ChId,
{
    /// Mutably borrow the source buffer
    pub(crate) fn borrow_source(&mut self) -> &mut BufferPairSrc<B> {
        self.buffers.source()
    }

    /// Mutable borrow the destination buffer
    pub(crate) fn borrow_destination(&mut self) -> &mut BufferPairDst<B> {
        self.buffers.destination()
    }

    /// Modify a completed transfer with new `source` and `destination` then restart.
    ///
    /// Returns a Result containing the source and destination from the
    /// completed transfer. Returns `Err(_)` if the buffer lengths are
    /// mismatched.
    #[inline]
    pub fn recycle(self, source: BufferPairSrc<B>, destination: BufferPairDst<B>) -> Result<(Transfer<B, Busy<Id>>, BufferPairSrc<B>, BufferPairDst<B>)> {
        let trig_src = self.trig_src;
        let trig_act = self.trig_act;
        let (chan, old_source, old_destination) = self.free();

        Self::check_buffer_pair(&source, &destination)?;

        // Circular transfers won't ever complete, so never recyle as one
        let new_transfer = unsafe { Transfer::<B, Ready<Id>>::new_unchecked(chan, source, destination, false, trig_src, trig_act) };

        Ok((new_transfer.begin(), old_source, old_destination))
    }

    /// Modify a completed transfer with a new `destination` then restart.
    ///
    /// Returns a Result containing the destination from the
    /// completed transfer. Returns `Err(_)` if the buffer lengths are
    /// mismatched.
    #[inline]
    pub fn recycle_source(self, destination: BufferPairDst<B>) -> Result<(Transfer<B, Busy<Id>>, BufferPairDst<B>)> {
        let trig_src = self.trig_src;
        let trig_act = self.trig_act;
        let (chan, old_source, old_destination) = self.free();

        Self::check_buffer_pair(&old_source, &destination)?;

        // Circular transfers won't ever complete, so never re-fill as one
        let new_transfer = unsafe{ Transfer::<B, Ready<Id>>::new_unchecked(chan, old_source, destination, false, trig_src, trig_act) };

        Ok((new_transfer.begin(), old_destination))
    }

    /// Modify a completed transfer with a new `source`, then restart.
    ///
    /// Returns a Result containing the source from the
    /// completed transfer. Returns `Err(_)` if the buffer lengths are
    /// mismatched.
    #[inline]
    pub fn recycle_destination(self, source: BufferPairSrc<B>) -> Result<(Transfer<B, Busy<Id>>, BufferPairSrc<B>)> {
        let trig_src = self.trig_src;
        let trig_act = self.trig_act;
        let (chan, old_source, old_destination) = self.free();

        Self::check_buffer_pair(&source, &old_destination)?;

        // Circular transfers won't ever complete, so never re-fill as one
        let new_transfer = unsafe{ Transfer::<B, Ready<Id>>::new_unchecked(chan, source, old_destination, false, trig_src, trig_act) };

        Ok((new_transfer.begin(), old_source))
    }

    /// Reset a completed transfer into a ready transfer using the same buffers
    pub fn reset(self) -> Transfer<B, Ready<Id>> {
        let trig_src = self.trig_src;
        let trig_act = self.trig_act;
        let (chan, source, destination) = self.free();

        // Always safe since we're using the same buffers
        unsafe { Transfer::<B, Ready<Id>>::new_unchecked(chan, source, destination, false, trig_src, trig_act) }
    }

    /// Release all owned resources
    #[inline]
    pub fn free(self) -> (Channel<Id, ChReady>, BufferPairSrc<B>, BufferPairDst<B>) {
        let buffers: SpecificBufferPair<B> = self.buffers.into();
        (
            self.state.0,
            buffers.source,
            buffers.destination,
        )
    }
}

//==============================================================================
// voltserver_hal DMA trait implementations
//==============================================================================
//impl<Chan, S, D> voltserver_hal::dma::Transfer for Transfer<Chan, BufferPair<S, D>>
//where
//    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
//    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
//    Chan: AnyChannel,
//{
//    type Error = super::Error;
//    type SrcWord = S::Beat;
//    type DstWord = D::Beat;
//    type Src = S;
//    type Dst = D;
//
//    fn source_buffer<'a>(&'a self) -> &'a Self::Src {
//        &self.buffers.source
//    }
//
//    fn dest_buffer<'a>(&'a self) -> &'a Self::Dst {
//        &self.buffers.destination
//    }
//}
//
//impl<C, S, D> voltserver_hal::dma::ReadyTransfer for Transfer<C, BufferPair<S, D>>
//where
//    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
//    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
//    C: AnyChannel<Status = Ready>,
//{
//    type Busy = Transfer<Channel<ChannelId<C>, Busy>, BufferPair<S, D>>;
//
//    fn begin(self) -> Result<Self::Busy> {
//        Ok(self.begin())
//    }
//}
//
//impl <C, S, D> voltserver_hal::dma::BusyTransfer for Transfer<C, BufferPair<S, D>>
//where
//    S: Buffer + voltserver_hal::dma::SrcBuffer<S::Beat>,
//    D: Buffer<Beat = S::Beat> + voltserver_hal::dma::DstBuffer<D::Beat>,
//    C: AnyChannel<Status = Busy>,
//{
//    type Ready = Transfer<Channel<ChannelId<C>, Ready>, BufferPair<S, D>>;
//
//    fn disable(self) -> Result<Self::Ready> {
//        let trig_src = self.trig_src;
//        let trig_act = self.trig_act;
//        let (channel, src, dst) = self.stop();
//
//        Ok(unsafe { Transfer::new_unchecked(channel, src, dst, false, trig_src, trig_act) })
//    }
//
//    fn software_trigger(&mut self) -> Result<()> {
//        self.software_trigger();
//        Ok(())
//    }
//
//    fn is_complete(&mut self) -> bool {
//        self.complete()
//    }
//}
