# Util Time User Guide

This guide explains how to use the time utils.

## tsc (Timestamp Converter)

The `tsc` application allows you to convert timestamps to different timezones.

```sh
tsc -- <TIMESTAMP> <TIMEZONE>

# Examples
tsc -- 1672531200 Australia/Sydney
tsc -- 1672531200 # default Australia/Sydney
```

## tzc (Timezone Converter)

The `tzc` application allows you to convert times between different timezones.

```sh
tzc -- <FROM_TIMEZONE> "<TIME>" <TO_TIMEZONE>

# Examples
tzc -- UTC "2023-01-01 12:00:00" Australia/Sydney
tzc -- UTC "2023-01-01 12:00:00" # default Australia/Sydney
```

## Conclusion

This guide provides a basic overview of how to use the time utilities such as `tsc` and `tzc` etc. For more detailed information, refer to the application's help command:

```sh
<APP> --help
```
