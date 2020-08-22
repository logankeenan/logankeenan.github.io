+++
title = "Automatically use the correct Node version when changing directories with NVM"
description = "Automatically use the correct Node version when changing directories with NVM"
date = 2019-10-17
+++

TLDR: Copy the script below to your .bash_profile (Mac) or .bashrc (Linux) to use the correct node version when starting
 a terminal session or changing directories.

There are plenty of tools that can do this for you, but bash is all you need. It will define a function which will check
 to see if a .nvmrc file exist and call nvm use if it does. I immediately invoke the function because the user might be 
 opening a new terminal session in a directory that contains an .nvmrc file. Otherwise, I hijack cd and call the 
 useNvmVersion function after a user has changed directories.
 
```bash
function useNvmVersion {
    if [ -f .nvmrc ]; then
        nvm use
    fi
}

useNvmVersion

function cdEnhanced {
    cd $1
    useNvmVersion
}

alias cd=cdEnhanced
```