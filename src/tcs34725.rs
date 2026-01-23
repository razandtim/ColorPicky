use embedded_hal::i2c::I2c;

pub const ADDRESS: u8 = 0x29;

pub const COMMAND_BIT: u8 = 0x80;

pub const REG_ENABLE: u8 = 0x00;
pub const REG_ATIME: u8 = 0x01;
pub const REG_CONTROL: u8 = 0x0F;
pub const REG_ID: u8 = 0x12;
pub const REG_CDATAL: u8 = 0x14;

pub const MASK_ENABLE_PON: u8 = 0x01; // Power ON
pub const MASK_ENABLE_AEN: u8 = 0x02; // RGBC Enable

pub struct Tcs34725<I2C> {
    i2c: I2C,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rgbc {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub c: u16,
}

impl<I2C, E> Tcs34725<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn init(&mut self) -> Result<(), E> {
        // Power ON and Enable RGBC
        let cmd = COMMAND_BIT | REG_ENABLE;
        self.i2c
            .write(ADDRESS, &[cmd, MASK_ENABLE_PON | MASK_ENABLE_AEN])?;

        // standard integration time (50ms) -> 0xEB?
        // 2.4ms * (256 - ATIME)
        // Let's use default or set something reasonable. 0xFF = 2.4ms, 0xC0 = 154ms
        // Let's set 50ms approx = 0xEB
        // self.write_reg(REG_ATIME, 0xEB)?;

        Ok(())
    }

    pub fn read_id(&mut self) -> Result<u8, E> {
        self.read_reg(REG_ID)
    }

    pub fn enable(&mut self) -> Result<(), E> {
        self.write_reg(REG_ENABLE, MASK_ENABLE_PON | MASK_ENABLE_AEN)
    }

    pub fn read_all(&mut self) -> Result<Rgbc, E> {
        let mut buf = [0u8; 8];
        let cmd = COMMAND_BIT | REG_CDATAL;

        self.i2c.write_read(ADDRESS, &[cmd], &mut buf)?;

        let c = u16::from_le_bytes([buf[0], buf[1]]);
        let r = u16::from_le_bytes([buf[2], buf[3]]);
        let g = u16::from_le_bytes([buf[4], buf[5]]);
        let b = u16::from_le_bytes([buf[6], buf[7]]);

        Ok(Rgbc { r, g, b, c })
    }

    fn write_reg(&mut self, reg: u8, value: u8) -> Result<(), E> {
        let cmd = COMMAND_BIT | reg;
        self.i2c.write(ADDRESS, &[cmd, value])
    }

    fn read_reg(&mut self, reg: u8) -> Result<u8, E> {
        let cmd = COMMAND_BIT | reg;
        let mut buf = [0u8; 1];
        self.i2c.write_read(ADDRESS, &[cmd], &mut buf)?;
        Ok(buf[0])
    }
}
