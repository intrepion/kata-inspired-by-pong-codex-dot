#!/usr/bin/env python3
"""Compare bounded Swift/Rust collision steps using a new output directory."""
import itertools
import pathlib
import random
import subprocess
import sys

if len(sys.argv) != 2:
    raise SystemExit('usage: python3 scripts/swift-rust-collision-parity/run.py NEW_OUTPUT_DIR')
root = pathlib.Path(__file__).resolve().parents[2]
out = pathlib.Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=False)
probe = pathlib.Path(__file__).resolve().parent
lib = out / 'libpong_core.rlib'
subprocess.run(['rustc', '--edition=2021', '--crate-name', 'pong_core', '--crate-type=lib', '-O', str(root/'crates/pong-core/src/lib.rs'), '-o', str(lib)], check=True)
subprocess.run(['rustc', '--edition=2021', '-O', str(probe/'probe.rs'), '--extern', f'pong_core={lib}', '-o', str(out/'rust-probe')], check=True)
subprocess.run(['swiftc', '-O', '-module-cache-path', str(out/'module-cache'), str(root/'experiments/native-swift/Sources/PongCore/Game.swift'), str(probe/'main.swift'), '-o', str(out/'swift-probe')], check=True)
rows=[]
for side,dt,y,vy,paddle,axis in itertools.product([0,1],[1/120,0.033],[8.3,10,45,180,195,197,200,215,250,295,300,303,306,320,455,490,491],[-600,-100,0,100,600],[45,250,455],[-1,0,1]):
    x=(54 if dt<0.01 else 60) if side==0 else (846 if dt<0.01 else 840)
    vx=-350 if side==0 else 350
    rows.append((x,y,vx,vy,paddle,paddle,dt,axis))
rng = random.Random(90210)
for _ in range(10000):
    side = rng.randrange(2)
    dt = rng.choice([1/120, 0.016, 0.033])
    speed = rng.uniform(120, 720)
    vx = -speed if side == 0 else speed
    face = 52 if side == 0 else 848
    contact_time = rng.uniform(0.001, 0.999) * dt
    x = face - vx * contact_time
    player, cpu = rng.uniform(45, 455), rng.uniform(45, 455)
    paddle = player if side == 0 else cpu
    offset = rng.choice([-1, 1]) * rng.uniform(35, 70)
    y = max(8, min(492, paddle + offset))
    vy = rng.uniform(-720, 720)
    axis = rng.choice([-1, 0, 1])
    rows.append((x,y,vx,vy,player,cpu,dt,axis))
text=''.join(','.join(map(str,row))+'\n' for row in rows)
(out/'cases.csv').write_text(text)
outputs=[]
for name in ['rust','swift']:
    r=subprocess.run([str(out/(name+'-probe'))],input=text,text=True,capture_output=True,check=True)
    (out/(name+'.csv')).write_text(r.stdout)
    outputs.append([[float(n) for n in line.split(',')] for line in r.stdout.splitlines()])
a,b=outputs
assert len(a)==len(b)==len(rows),(len(a),len(b),len(rows))
maxima=[max(abs(x[i]-y[i]) for x,y in zip(a,b)) for i in range(6)]
sign=[i for i,(x,y) in enumerate(zip(a,b)) if (x[2]>0)!=(y[2]>0)]
outliers=[i for i,(x,y) in enumerate(zip(a,b)) if max(abs(u-v) for u,v in zip(x,y))>0.001]
print('cases',len(rows),'max abs per [x,y,vx,vy,player,cpu]',maxima,'direction mismatches',len(sign),'above tolerance 0.001',len(outliers))
for i in outliers[:3]: print('outlier',i,rows[i],a[i],b[i])
if sign or outliers: raise SystemExit(1)
