#![no_std] //Desabilitamos la compilacion o generacion con librerias estandar
#![no_main] //Sobreescribe el punto de entrada main
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


mod vga_buffer;

use core::panic::PanicInfo;

//PanicInfo contiene el archivo y linea donde se creo el error
//La funcion nunca debe de retornar, asi que se marca como funcion divergente (! - never type)

#[panic_handler]
fn panic(info: &PanicInfo) -> ! { 
    println!("Error info: {}", info);
    loop {} 
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]){
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial...");
    assert_eq!(1,1);
    println!("[Ok]");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //use core::fmt::Write;

    //vga_buffer::WRITER.lock().write_str("Hola de nuevo").unwrap();
    //write!(vga_buffer::WRITER.lock(), ", algunos numeros {}, {}", 42, 15.7).unwrap();
    println!("Hola mundo{}", "!");

    //panic!("Panic error!!"); //Panic aborta en ese momento

    #[cfg(test)]
    test_main();

    loop {}
}

