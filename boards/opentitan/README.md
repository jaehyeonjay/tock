OpenTitan RISC-V Board
======================

- https://opentitan.org/

OpenTitan is the first open source project building a transparent,
high-quality reference design and integration guidelines for
silicon root of trust (RoT) chips.

Tock currently supports OpenTitan on the Nexys Video and the ChipWhisperer
CW310 FPGA boards. For more details on the boards see:
https://docs.opentitan.org/doc/ug/fpga_boards/

You can get started with OpenTitan using either the Nexys Video FPGA
board, ChipWhisperer CW310 board or a simulation. See the OpenTitan
[getting started](https://docs.opentitan.org/doc/ug/getting_started/index.html)
for more details.

Programming
-----------

Tock on OpenTitan requires
lowRISC/opentitan@7e60eca10f23f5a4fcdc3723fc571599f8e00178 or newer. In
general it is recommended that users start with the latest OpenTitan bitstream
and if that results in issues try the one mentioned above.

For more information you can follow the
[OpenTitan development flow](https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html#testing-the-demo-design)
to flash the image.

First setup the development board using the steps here:
https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html.
You need to make sure the boot ROM is working and that your machine can
communicate with the OpenTitan ROM. You will need to use the `PROG` USB
port on the board for this.

Nexys Video
-----------

To use `make flash` you first need to clone the OpenTitan repo and build
the `spiflash` tool.

In the OpenTitan repo build the `spiflash` program.

```shell
./meson_init.sh
ninja -C build-out sw/host/spiflash/spiflash_export
```

Export the `OPENTITAN_TREE` enviroment variable to point to the OpenTitan tree.

```shell
export OPENTITAN_TREE=/home/opentitan/
```

Back in the Tock board directory run `make flash`

If everything works you should see something like this on the console.
If you need help getting console access check the
[testing the design](https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html#testing-the-demo-design)
section in the OpenTitan documentation.

```
Bootstrap: DONE!
Boot ROM initialisation has completed, jump into flash!
OpenTitan initialisation complete. Entering main loop
```

You can also just use the `spiflash` program manually to download the image
to the board if you don't want to use `make flash`.

```shell
./sw/host/spiflash/spiflash --input=../../../target/riscv32imc-unknown-none-elf/release/opentitan.bin
```

NOTE: You will need to download the Tock binary after every power cycle.

ChipWhisper CW310
-----------------

To use `make flash` you first need to clone the OpenTitan repo and ensure that
the Python dependencies are installed.

```shell
python3 pip install -r python-requirements.txt
```

Next connect to the boards serieal with a second terminal:

```shell
screen /dev/ttyACM1 115200,cs8,-ixon,-ixoff
```

Then you need to flash the bitstream with:


```shell
./util/fpga/cw310_loader.py --bitstream lowrisc_systems_chip_earlgrey_cw310_0.1.bit --set-pll-defaults
```

After which you should see some output in the serial window.

Then in the Tock board directoty export the `OPENTITAN_TREE` enviroment
variable to point to the OpenTitan tree.

```shell
export OPENTITAN_TREE=/home/opentitan/
```

then you can run `make flash` or `make test-hardware` to use the board.

### Compiling the Kernel for FPGA or Verilator

Opentitan is supported on both an FPGA and in Verilator. Slightly different
versions of the EarlGrey chip implementation are required for the different
platforms. By default the kernel is compiled for the FPGA. To compile for
Verilator, run:

```shell
make BOARD_CONFIGURATION=sim_verilator
```

To explicitly specify the FPGA, run:

```shell
make BOARD_CONFIGURATION=fpga_nexysvideo
```

Programming Apps
----------------

Tock apps for OpenTitan must be included in the Tock binary file flashed with
the steps mentioned above.

Apps are built out of tree.

The OpenTitan Makefile can also handle this process automatically. Follow
the steps above but instead run the `flash-app` make target.

```shell
$ make flash-app APP=<...> OPENTITAN_TREE=/home/opentitan/
```

You will need to have the GCC version of RISC-V 32-bit objcopy installed as
the LLVM one doesn't support updating sections.

Running in QEMU
---------------

The OpenTitan application can be run in the QEMU emulation platform for
RISC-V, allowing quick and easy testing. This is also a good option for
those who can't afford the FPGA development board.

Unfortunately you need QEMU 6.1, which at the time of writing is unlikely
to be avaliable in your distro. Luckily Tock can build QEMU for you. From
the top level of the Tock source just run `make ci-setup-qemu` and
follow the steps.

QEMU can be started with Tock using the `qemu` make target:

```shell
$ make OPENTITAN_BOOT_ROM=<path_to_opentitan>/sw/device/boot_rom/boot_rom_fpga_nexysvideo.elf qemu
```

Where OPENTITAN_BOOT_ROM is set to point to the OpenTitan ELF file. This is
usually located at `sw/device/boot_rom/boot_rom_fpga_nexysvideo.elf` in the
OpenTitan build output. Note that the `make ci-setup-qemu` target will also
download a ROM file.

QEMU can be started with Tock and a userspace app with the `qemu-app` make
target:

```shell
$ make OPENTITAN_BOOT_ROM=<path_to_opentitan/sw/device/boot_rom/boot_rom_fpga_nexysvideo.elf> APP=/path/to/app.tbf qemu-app
```

The TBF must be compiled for the OpenTitan board. For example, you can build
the Hello World exmple app from the libtock-rs repository by running:

```
$ cd [LIBTOCK-RS-DIR]
$ make flash-opentitan
$ tar xf target/riscv32imac-unknown-none-elf/tab/opentitan/hello_world.tab
$ cd [TOCK_ROOT]/boards/opentitan
$ make APP=[LIBTOCK-RS-DIR]/rv32imac.tbf qemu-app
```
