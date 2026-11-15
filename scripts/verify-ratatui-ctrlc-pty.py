import os,pty,fcntl,termios,struct,subprocess,select,time,sys,signal
binary=sys.argv[1]
for mode in ('enhanced','fallback'):
 m,s=pty.openpty();fcntl.ioctl(s,termios.TIOCSWINSZ,struct.pack('HHHH',30,100,0,0)); before=termios.tcgetattr(s)
 p=subprocess.Popen([binary],stdin=s,stdout=s,stderr=s,start_new_session=True,env={**os.environ,'TERM':'xterm-256color'});data=b''; replied=False;sent=False;deadline=time.monotonic()+8
 try:
  while time.monotonic()<deadline and p.poll() is None:
   if select.select([m],[],[],0.05)[0]:
    try:data+=os.read(m,65536)
    except OSError:break
   if not replied and b'\x1b[?u\x1b[c' in data:
    os.write(m,b'\x1b[?1u\x1b[?1;2c' if mode=='enhanced' else b'\x1b[?1;2c');replied=True
   if replied and not sent and b'\x1b[?1049h' in data:
    os.write(m,b'\x1b[99;5u' if mode=='enhanced' else b'\x03');sent=True
  assert replied and sent,(mode,'probe or start not reached')
  status=p.wait(timeout=2)
  assert status==0,(mode,status)
  assert termios.tcgetattr(s)==before,(mode,'terminal attributes not restored')
  while select.select([m],[],[],0)[0]:
   try:data+=os.read(m,65536)
   except OSError:break
  assert b'\x1b[?1049l' in data,(mode,'no alternate-screen exit')
  print('PASS',mode,'Ctrl+C exits 0, restores terminal attributes and leaves alternate screen')
 finally:
  if p.poll() is None:
   os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=2)
  os.close(m);os.close(s)
