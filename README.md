# Tenma Program

## Usage

This program takes a lua script and a number of serial ports as command line inputs.

e.g.
```bash
./program_name test.lua /dev/tty.Port
```
For MacOs and Linux. Or
```bash
program_name test.lua COM10
```
For Windows.


The serial ports can be referenced by their index with the first serial port starting with an index of `1`.

If a serial port is replaced to `tester`, all commands sent to that serial port will be instead printed to the screen. Multiple `tester`'s can be set.

## Lua Syntax

### Configuration Functions
- #### ch(channel)
    - **channel**: Sets the selected channel, all following commands will be sent on the selected channel. All Tenma Functions default to channel `1` if not set.
    - e.g.
        ```lua
        v(3)
        ch(2) v(5)
        ```
        Will set channel `1` to 3 volts, then set channel `2` to 5 volts.
- #### ser(serial)
    - **serial**: Sets the selected serial port, all following commands will be sent on the selected serial port. All Tenma Functions defaults to serial port `1` if not set.

### Available Tenma Functions
- #### v(voltage, delay (optional))
    - Will set the tenma to output a set voltage.
    - **voltage**: Measured in volts, can be a decimal number.
    - **delay**: Measured in seconds, can be a decimal number.
- #### i(current, delay (optional))
    - Will set the tenma to have a max output of `current` amps.
    - **current**: Measured in amps, can be a decimal number.
    - **delay**: Measured in seconds, can be a decimal number.
- #### out(on)
    - **on**: Can either be `true` or `false`, used to turn the output on or off.
- #### getv()
    - Returns the measured voltage in volts.
    - Will return 0 if run through the tester.
    - e.g.
        ```lua
        volts = getv()
        print(volts)
        ```
        Will output ```3``` if the tenma is set to 3 volts and its out is set to `true`.
- #### geti()
    - Returns the measured current in amps.
    - Will return 0 if run through the tester.

### Available Utility Functions
- #### range(start, end, step (optional))
    - Creates an iterator over the set range with an optional step. 
    - Start and end are both inclusive. 
    - Direction of range is determined by the size of start and end, not the sign of the step. If end is greater than the start, then the iterator will count up, otherwise it will count down.
    - e.g
        ```lua
        for volts in range(8, 5, 0.5) do
            print(volts)
        end
        ```
        Will output:
        ```
        8
        7.5
        7
        6.5
        6
        5.5
        5
        ```
- #### delay(delay)
    - Will pause the execution by `delay` seconds.