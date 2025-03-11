use super::i2c;

use super::regs::i2c::*;

pub fn read(buf: &mut [u8], mut len: u32, offset: u32) {
    if buf.len() < len as usize {
        len = buf.len() as u32;
    }
    i2c::set_data_count(2);
    i2c::master_int_clear_ex(0x7FF);
    i2c::master_control(I2C_CFG_MST_TX);
    i2c::master_start();

    while !i2c::master_bus_busy() {}

    i2c::master_data_put(((offset >> 8) & 0xFF) as u8);

    while !i2c::master_int_raw_status_ex(I2C_INT_TRANSMIT_READY) {}

    i2c::master_data_put((offset & 0xFF) as u8);
    i2c::master_int_clear_ex(I2C_INT_TRANSMIT_READY);

    while i2c::master_int_raw_status() & I2C_INT_ADRR_READY_ACESS == 0 {}
    i2c::master_int_clear_ex(0x7FF);

    i2c::set_data_count(len);
    i2c::master_control(I2C_CFG_MST_RX);
    i2c::master_start();

    let mut idx = 0;
    while len > 0 {
        while !i2c::master_int_raw_status_ex(I2C_INT_RECEIVE_READY) {}
        buf[idx] = i2c::master_data_get();
        i2c::master_int_clear_ex(I2C_INT_RECEIVE_READY);

        len -= 1;
        idx += 1;
    }

    i2c::master_stop();
    while i2c::master_int_raw_status() & I2C_INT_STOP_CONDITION == 0 {}
    i2c::master_int_clear_ex(I2C_INT_STOP_CONDITION);
}

pub fn init(slave: u8) {
    i2c::init_clocks();
    i2c::mux_pins(0);
    i2c::master_disable();
    i2c::soft_reset();
    i2c::master_init_clock(I2C_SYSTEM_CLOCK, I2C_INTERNAL_CLOCK * 2, I2C_OUTPUT_CLOCK);
    i2c::master_slave_addr_set(slave);
    i2c::master_int_disable_ex(0xFFFFFFFF);
    i2c::master_enable();
}
