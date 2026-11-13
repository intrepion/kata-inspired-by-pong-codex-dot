#!/usr/bin/env python3
"""Exercise a Ratatui binary with synthetic enhanced-key events in a PTY.

Usage: python3 scripts/verify-ratatui-enhanced-pty.py BINARY RAW_OUTPUT
RAW_OUTPUT must not exist. This is not a real-terminal or physical-input test.
"""
import sys
import os,pty,fcntl,termios,struct,subprocess,select,time,codecs,re
if len(sys.argv) != 3:
 raise SystemExit(__doc__)
binary, raw_output = sys.argv[1:]
if os.path.exists(raw_output):
 raise SystemExit('Refusing to overwrite raw output: ' + raw_output)
m,s=pty.openpty(); fcntl.ioctl(s,termios.TIOCSWINSZ,struct.pack('HHHH',30,100,0,0)); before=termios.tcgetattr(s)
p=subprocess.Popen([binary],stdin=s,stdout=s,stderr=s,start_new_session=True,env={**os.environ,'TERM':'xterm-256color'})
log=b''; buf=''; dec=codecs.getincrementaldecoder('utf-8')('replace'); screen=[[' ']*100 for _ in range(30)]; row=col=0; query=False

def pump(duration):
 global log,buf,row,col,query
 end=time.monotonic()+duration
 while time.monotonic()<end:
  if not select.select([m],[],[],.03)[0]: continue
  try: data=os.read(m,65536)
  except OSError: break
  if not data: break
  log+=data
  if not query and b'\x1b[?u\x1b[c' in log:
   os.write(m,b'\x1b[?1u\x1b[?1;2c'); query=True
  buf+=dec.decode(data)
  while buf:
   if buf[0]=='\x1b':
    if len(buf)<2: break
    if buf[1]=='[':
     match=re.match(r'\x1b\[([0-9;?<>:]*)([ -/]*)([@-~])',buf)
     if not match: break
     args,_,cmd=match.groups(); buf=buf[match.end():]
     try: nums=[int(x or 0) for x in args.split(';')]
     except ValueError: continue
     if cmd in 'Hf': row=max(0,(nums[0] if nums else 1)-1); col=max(0,(nums[1] if len(nums)>1 else 1)-1)
     elif cmd=='C': col+=nums[0] or 1
     elif cmd=='D': col=max(0,col-(nums[0] or 1))
     elif cmd=='A': row=max(0,row-(nums[0] or 1))
     elif cmd=='B': row+=nums[0] or 1
     elif cmd=='J' and nums[0]==2: screen[:]=[[' ']*100 for _ in range(30)]
     elif cmd=='K' and row<30: screen[row][col:]=[' ']*(100-col)
     continue
    buf=buf[2:]; continue
   ch=buf[0]; buf=buf[1:]
   if ch=='\r': col=0
   elif ch=='\n': row=min(29,row+1)
   elif ch=='\b': col=max(0,col-1)
   elif ord(ch)>=32:
    if row<30 and col<100: screen[row][col]=ch
    col+=1

def paddle():
 return tuple((i, ''.join(line[5:7])) for i,line in enumerate(screen) if any(c in '█▄▀' for c in line[5:7]))
def state(expected):
 assert expected in ''.join(screen[0]), (expected, ''.join(screen[0]))
def send(seq,secs=.14): os.write(m,seq); pump(secs)
try:
 pump(.5)
 assert query and 'Hold' in ''.join(screen[1]), 'enhanced keyboard path unavailable'
 state('Ready'); initial=paddle()
 send(b'\x1b[32;1:1u'); state('Playing')
 send(b'\x1b[119;1:1u',.17); moving=paddle()
 assert moving and moving != initial and moving[0][0] < initial[0][0], (initial,moving)
 send(b'\x1b[32;1:1u'); state('Paused'); paused=paddle()
 send(b'\x1b[32;1:1u'); state('Playing')
 send(b'\x1b[119;1:2u',.22); repeated=paddle()
 assert repeated == paused, (paused,repeated)
 send(b'\x1b[119;1:3u')
 send(b'\x1b[119;1:1u',.17); fresh=paddle()
 assert fresh != repeated and fresh[0][0] < repeated[0][0], (repeated,fresh)
 send(b'\x1b[119;1:3u'); released=paddle(); pump(.2)
 assert paddle() == released, (released,paddle())
 send(b'\x1b[113;1:1u'); assert p.wait(timeout=2)==0
 assert termios.tcgetattr(s)==before, 'terminal attributes not restored'
 print('PASS enhanced protocol; ready -> playing -> paused -> playing; W press moved; held repeat after pause stayed stopped; fresh press moved; release stopped; Q exited; termios restored')
 print('player paddle top rows:',initial[0][0],moving[0][0],paused[0][0],repeated[0][0],fresh[0][0])
finally:
 if p.poll() is None: p.terminate(); p.wait(timeout=2)
 with open(raw_output, 'xb') as output: output.write(log)
