#[doc = "Register `IDR_SPI_MODE` writer"]
pub type W = crate::W<SpiModeIdrSpiModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - "]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - "]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Disable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - "]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - "]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<'_, SpiModeIdrSpiModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<'_, SpiModeIdrSpiModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn endrx(&mut self) -> EndrxW<'_, SpiModeIdrSpiModeSpec> {
        EndrxW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn endtx(&mut self) -> EndtxW<'_, SpiModeIdrSpiModeSpec> {
        EndtxW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<'_, SpiModeIdrSpiModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, SpiModeIdrSpiModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UnreW<'_, SpiModeIdrSpiModeSpec> {
        UnreW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txbufe(&mut self) -> TxbufeW<'_, SpiModeIdrSpiModeSpec> {
        TxbufeW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RxbuffW<'_, SpiModeIdrSpiModeSpec> {
        RxbuffW::new(self, 12)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mode_idr_spi_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiModeIdrSpiModeSpec;
impl crate::RegisterSpec for SpiModeIdrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mode_idr_spi_mode::W`](W) writer structure"]
impl crate::Writable for SpiModeIdrSpiModeSpec {
    type Safety = crate::Unsafe;
}
