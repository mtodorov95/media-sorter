# rust-media-sorter 

## TODO
- Add a help command

## Summary
A simple file sorting application for when you have many individual files in a folder that need to be grouped in their
own folders. I use it for videos but it should work just the same for other file types as well.
Should work on both Linux and Mac.

## Config

You can pass the following command line args:
```
-src/--s - The directory to be used as a source for the files (Default: $HOME/Downloads)
-target/--t - The directory to put the files into (Default: $HOME/Videos)
-ext/--e - The extension for the files to be sorted (Default: mp4)
```

To change the default folders set the following env variables to whatever you need:
```
SORTER_SRC_DIR
SORTER_TARGET_DIR
```
Order of priority is: Command line arg > ENV variable > Default

