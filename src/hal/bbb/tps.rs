use super::i2c::{self, I2C_INT_TRANSMIT_READY};

pub const MASK_ALL_BITS: u32 = 0xFF;

pub const PASSWORD_LOCK_FOR_WRITE: u8 = 0x00;
pub const PASSWORD_UNLOCK: u8 = 0x7D;
pub const PMIC_TPS65217_I2C_SLAVE_ADDR: u8 = 0x24;

pub const PASSWORD: u8 = 0x0B;

fn cleanup_interrupt() {
    i2c::master_int_clear_ex(i2c::I2C_INTERRUPT_FLAG_TO_CLR);
}

fn setup_reception(offset: u8, mut dcount: u32, buffer: &mut [u8]) -> u32 {
    let mut received = 0;

    i2c::set_data_count(1);
    cleanup_interrupt();
    i2c::master_control(i2c::I2C_CFG_MST_TX);
    i2c::master_start();
    while !i2c::master_bus_busy() {}

    i2c::master_data_put(offset);
    i2c::master_int_clear_ex(I2C_INT_TRANSMIT_READY);
    while (i2c::master_int_raw_status() & i2c::I2C_INT_ADRR_READY_ACESS) == 0 {}

    i2c::set_data_count(dcount);
    cleanup_interrupt();
    i2c::master_control(i2c::I2C_CFG_MST_RX);
    i2c::master_start();

    while dcount > 0 {
        while !i2c::master_int_raw_status_ex(i2c::I2C_INT_RECEIVE_READY) {}
        buffer[received] = i2c::master_data_get();
        i2c::master_int_clear_ex(i2c::I2C_INT_RECEIVE_READY);
        dcount -= 1;
        received += 1;
    }

    i2c::master_stop();
    while i2c::master_int_raw_status() & i2c::I2C_INT_STOP_CONDITION == 0 {}
    i2c::master_int_clear_ex(i2c::I2C_INT_STOP_CONDITION);

    received as u32
}

pub fn config_vdd_op_voltage() {
    let mut pmic = 0;
    i2c::master_slave_addr_set(PMIC_TPS65217_I2C_SLAVE_ADDR);
}

pub fn read_reg(offset: u8) -> u8 {
    let mut buffer = [0u8; 1];
    setup_reception(offset, 1, &mut buffer);
    buffer[0]
}

pub fn write_reg(port_level: u32, offset: u8, mut dest_val: u32, mask: u32) {
    let mut buffer = [0u8; 1];

    if mask != MASK_ALL_BITS {
        let mut received = setup_reception(offset, 1, &mut buffer);
        received &= !mask;
        received |= dest_val & mask;
        dest_val = received;
    }

    if port_level > 0 {
        let xor_reg = offset ^ PASSWORD_UNLOCK;
        let mut buffer = [0u8; 2];
        buffer[0] = PASSWORD;
        buffer[1] = xor_reg;
    }
}
