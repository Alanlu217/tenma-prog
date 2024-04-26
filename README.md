# Tenma Program

## Tenma Script Syntax

### Tenma Interactions

#### Current

I {current in amps}

If there is no delay following this command, a 50 millisecond delay will automatically be added.

#### Voltage

V {voltage in volts}

If there is no delay following this command, a 50 millisecond delay will automatically be added.

#### Turning off

OFF

### Delay

: {delay} {units}

units are either 'min' minutes, 's' seconds or 'ms' milliseconds. Default unit is in seconds

can be placed directly after a tenma command in the same line. e.g. 
```
V 10 : 10 ms
```

### Loops

loop {number of loops}
...
end

if nothing is specified, loop will run forever.
