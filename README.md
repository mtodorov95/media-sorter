# media-sorter 

## Summary
A simple file sorting application for when you have many individual files in a folder that need to be grouped in their
own folders. I use it for videos but it should work just the same for other file types as well.
Should work on both Linux and Mac.

## Config

You can pass the following command line args:
```
-s/--src - The directory to be used as a source for the files (Default: $HOME/Downloads)
-t/--target - The directory to put the files into (Default: $HOME/Videos)
-e/--ext - The extensions for the files to be sorted as a space separated list (Default: mp4)
-k - Keeps the original file name (By default any prefixes - "[prefix] filename.mp4" - are removed)
```

To change the default folders set the following env variables to whatever you need:
```
SORTER_SRC_DIR
SORTER_TARGET_DIR
```
Order of priority is: Command line arg > ENV variable > Default

