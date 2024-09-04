; Run sudo
SetTimer, RunSudo, 5000
return

RunSudo:
    Run, %ComSpec% /c sudo -v,, Hide
return
