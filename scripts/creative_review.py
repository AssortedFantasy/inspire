"""Build an offline, initially blinded review of frozen creative comparisons.

python -B scripts/creative_review.py --base target/creative-validation
"""

import argparse
import base64
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def checked_path(base, relative):
    path = (base / relative).resolve()
    if not path.is_relative_to(base.resolve()):
        raise ValueError(f"Path outside study directory: {relative}")
    return path


def collect(base):
    manifest_path = base / "manifest.json"
    manifest = load(manifest_path)
    key = load(base / "cases-key.json")
    if key["status"] != "frozen" or sha(manifest_path.read_bytes()) != key["manifest_sha256"]:
        raise ValueError("Frozen manifest does not match cases key")
    jobs = {job["id"]: job for job in manifest["jobs"]}
    entries = {entry["id"]: entry for entry in key["cases"]}
    pairs = []
    for pair in key["pairs"]:
        originals = [entries[name] for name in pair["cases"] if not entries[name]["mirrored"]]
        if len(originals) != 1:
            raise ValueError(f"Expected one original case for {pair['id']}")
        entry = originals[0]
        if entry["id"] in key["controls"] or entry["pair_id"] != pair["id"]:
            raise ValueError("Control or unrelated case in comparison pairs")
        path = checked_path(base, entry["file"])
        if sha(path.read_bytes()) != entry["sha256"]:
            raise ValueError(f"Frozen case changed: {entry['id']}")
        case = load(path)
        streams = {}
        for side in ("A", "B"):
            if len(case["streams"][side]) != 10:
                raise ValueError("Each comparison must contain ten works per side")
            works = []
            for index, row in enumerate(case["streams"][side], 1):
                label = f"{side}{index}"
                if row["label"] != label:
                    raise ValueError("Unexpected item order")
                job_id = entry["item_jobs"][label]
                job = jobs[job_id]
                record = load(checked_path(base, f"records/{job_id}.json"))
                if sha(canonical(record)) != key["generation_sha256"][job_id]:
                    raise ValueError(f"Frozen output changed: {job_id}")
                if (job["task"], job["replicate"], job["condition"]) != (
                    pair["task"], pair["replicate"], entry["sides"][side]
                ):
                    raise ValueError("Item provenance does not match comparison")
                work = {"label": label, "text": record["output_text"],
                        "id": job_id, "status": record["status"],
                        "inspiration": job["inspiration"]}
                if "image" in row:
                    prefix = "data:image/png;base64,"
                    if not row["image"].startswith(prefix):
                        raise ValueError("Review images must be embedded PNGs")
                    raw = base64.b64decode(row["image"][len(prefix):], validate=True)
                    if not raw.startswith(b"\x89PNG\r\n\x1a\n"):
                        raise ValueError("Invalid PNG image")
                    rendered = checked_path(base, f"rendered/{job_id}.png")
                    if raw != rendered.read_bytes():
                        raise ValueError(f"Rendered image changed: {job_id}")
                    work["image"] = row["image"]
                elif row["text"] != record["output_text"]:
                    raise ValueError(f"Case output differs from original: {job_id}")
                works.append(work)
            streams[side] = works
        judgments = []
        for case_id in pair["cases"]:
            judgment_path = checked_path(base, f"judgments/{case_id}.json")
            if not judgment_path.exists():
                continue
            judgment = load(judgment_path)
            if judgment.get("request_sha256") != entries[case_id]["request_sha256"]:
                raise ValueError(f"Judgment request changed: {case_id}")
            try:
                verdict = json.loads(judgment["output_text"])
            except (ValueError, KeyError):
                verdict = None
            fields = {"case_id", "diversity_score", "quality_score", "task_failures",
                      "a_redundant_groups", "b_redundant_groups", "reason"}
            if not isinstance(verdict, dict) or set(verdict) != fields:
                verdict = None
            judgments.append({"id": case_id, "mirrored": entries[case_id]["mirrored"],
                              "status": judgment["status"], "verdict": verdict,
                              "text": judgment.get("output_text", "")})
        pairs.append({"id": pair["id"], "task": pair["task"], "replicate": pair["replicate"],
                      "prompt": case["task"], "sides": entry["sides"],
                      "streams": streams, "judgments": judgments})
    if not pairs:
        raise ValueError("No frozen comparison pairs")
    return {"pairs": pairs}


HTML = r"""<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data:; style-src 'unsafe-inline'; script-src 'unsafe-inline'; connect-src 'none'; base-uri 'none'; form-action 'none'">
<title>Creative output review</title>
<style>
*{box-sizing:border-box}body{margin:0 auto;padding:22px 24px 60px;max-width:1400px;color:#202124;background:#fff;font:15px/1.5 system-ui,sans-serif}
h1{font-size:23px;line-height:1.3;margin:0 0 16px}h2{font-size:18px;margin:0}h3{font-size:14px;margin:0 0 8px;font-weight:650}
button,select{font:inherit;color:inherit;border:1px solid #bfc3c8;border-radius:3px;background:#fff;padding:5px 9px}button{cursor:pointer}button:disabled{cursor:default;color:#74777b;background:#f6f6f6}
label{display:inline-flex;align-items:center;gap:7px}nav{display:flex;align-items:center;gap:12px;flex-wrap:wrap}#count{color:#60646a;font-size:13px}#prompt{margin:15px 0;white-space:pre-wrap}#actions{display:flex;gap:16px;align-items:center;flex-wrap:wrap;margin:0 0 17px}
.columns,.row{display:grid;grid-template-columns:minmax(0,1fr) minmax(0,1fr);gap:28px}.columns{border-bottom:2px solid #d7d9dc;padding-bottom:9px}.row{border-bottom:1px solid #ddd;padding:18px 0}.work{min-width:0}.text{white-space:pre-wrap;overflow-wrap:anywhere;margin:0;font:inherit}.picture{display:block;width:100%;max-width:520px;height:auto;aspect-ratio:1;object-fit:contain;background:white}
details{margin-top:10px;font-size:13px}summary{cursor:pointer;color:#50565c}details pre{white-space:pre-wrap;overflow-wrap:anywhere;font:12px/1.5 ui-monospace,monospace}#judgments{border-top:1px solid #ccc;border-bottom:1px solid #ccc;margin-bottom:20px;padding:12px 0}#judgments p{margin:5px 0 12px}.muted{color:#60646a}.condition{font-size:14px;font-weight:400}button:focus-visible,select:focus-visible,summary:focus-visible,input:focus-visible{outline:2px solid #3568b0;outline-offset:3px}
@media(max-width:650px){body{padding:16px 12px 40px}.columns,.row{gap:16px}nav{gap:9px}.text{font-size:14px}.picture{min-width:0}}
</style></head><body>
<h1>Creative output review</h1>
<nav aria-label="Comparison selection">
<label>Task <select id="task"></select></label>
<label>Replicate <select id="replicate"></select></label>
<button id="previous" type="button">Previous</button><button id="next" type="button">Next</button>
<span id="count" aria-live="polite"></span>
</nav>
<p id="prompt"></p>
<div id="actions"><button id="reveal" type="button">Reveal conditions</button>
<label><input id="showJudges" type="checkbox" disabled>Show judge assessments</label></div>
<section id="judgments" aria-label="Judge assessments" hidden></section>
<div class="columns"><h2 id="headingA">A</h2><h2 id="headingB">B</h2></div>
<main id="works" aria-label="Creative works"></main>
<script id="data" type="application/json">__DATA__</script>
<script>
const data=JSON.parse(document.getElementById('data').textContent), pairs=data.pairs;
const el=id=>document.getElementById(id), task=el('task'), replicate=el('replicate');
const taskNames={poem:'Poem',game:'Video game pitch',svg:'SVG artwork',microfiction:'Microfiction'};
const conditions={inspire:'Random inspiration',control:'Bare task'};
const revealed=new Set();let current=0;
function node(tag,text,cls){const x=document.createElement(tag);if(text!==undefined)x.textContent=text;if(cls)x.className=cls;return x}
function option(select,value,label){const x=node('option',label);x.value=value;select.append(x)}
for(const name of [...new Set(pairs.map(p=>p.task))])option(task,name,taskNames[name]||name);
function fillReplicates(value){replicate.replaceChildren();for(const p of pairs.filter(p=>p.task===task.value))option(replicate,p.replicate,p.replicate);if(value!==undefined)replicate.value=value}
function detail(parent,title,text){const d=node('details');d.append(node('summary',title),node('pre',text));parent.append(d)}
function provenance(work){const info={id:work.id,status:work.status};if(work.inspiration){info.seed=work.inspiration.seed;info.inspiration=work.inspiration.payload}else info.inspiration=null;return JSON.stringify(info,null,2)}
function showWork(work,isRevealed){const article=node('article',undefined,'work');article.append(node('h3',work.label));if(work.image){const img=node('img',undefined,'picture');img.src=work.image;img.alt=work.label+' rendered SVG';img.width=400;img.height=400;article.append(img);detail(article,'SVG source',work.text)}else article.append(node('div',work.text,'text'));if(isRevealed)detail(article,'Seed and inspiration',provenance(work));return article}
function swapLabels(text){return String(text).replace(/\b([AB])(\d+)\b/g,(_,side,n)=>(side==='A'?'B':'A')+n)}
function alignedVerdict(judgment){
  const v=JSON.parse(JSON.stringify(judgment.verdict));
  if(!judgment.mirrored)return v;
  v.diversity_score=-v.diversity_score;
  v.quality_score=-v.quality_score;
  v.reason=swapLabels(v.reason);
  [v.a_redundant_groups,v.b_redundant_groups]=[v.b_redundant_groups,v.a_redundant_groups];
  for(const group of [...v.a_redundant_groups,...v.b_redundant_groups]){
    group.items=group.items.map(swapLabels);
    group.reason=swapLabels(group.reason);
  }
  for(const failure of v.task_failures){
    failure.item=swapLabels(failure.item);
    failure.reason=swapLabels(failure.reason);
  }
  return v;
}
function showJudgments(){
  const p=pairs[current],box=el('judgments');
  box.replaceChildren();
  box.hidden=!el('showJudges').checked||!revealed.has(p.id);
  if(box.hidden)return;
  if(!p.judgments.length){box.append(node('p','No judge assessments available.'));return}
  box.append(node('p','Scores, groups and item labels below use displayed A/B.','muted'));
  for(const j of p.judgments){
    box.append(node('h3',j.mirrored?'Mirrored judgment':'Original judgment'));
    if(!j.verdict){
      box.append(node('p','No parsed assessment ('+j.status+').'));
      detail(box,'Raw judgment',j.text);
      continue;
    }
    const v=alignedVerdict(j);
    box.append(node('p','Diversity: '+v.diversity_score+' · Quality: '+v.quality_score+' (positive favors displayed A)'));
    box.append(node('p',v.reason));
    detail(box,'Full assessment aligned to displayed A/B',JSON.stringify(v,null,2));
    detail(box,'Raw judgment in its original A/B orientation',j.text);
  }
}
function render(){const p=pairs[current],visible=revealed.has(p.id);task.value=p.task;fillReplicates(p.replicate);el('count').textContent=(current+1)+' / '+pairs.length;el('prompt').textContent=p.prompt;el('previous').disabled=current===0;el('next').disabled=current===pairs.length-1;el('reveal').disabled=visible;el('reveal').textContent=visible?'Conditions revealed':'Reveal conditions';el('showJudges').disabled=!visible;for(const side of ['A','B']){const h=el('heading'+side);h.replaceChildren(node('span',side));if(visible)h.append(node('span',' — '+(conditions[p.sides[side]]||p.sides[side]),'condition'))}const works=el('works');works.replaceChildren();for(let i=0;i<10;i++){const row=node('div',undefined,'row');row.append(showWork(p.streams.A[i],visible),showWork(p.streams.B[i],visible));works.append(row)}showJudgments()}
function navigate(index){current=index;el('showJudges').checked=false;render();window.scrollTo(0,0)}
task.addEventListener('change',()=>navigate(pairs.findIndex(p=>p.task===task.value)));
replicate.addEventListener('change',()=>navigate(pairs.findIndex(p=>p.task===task.value&&String(p.replicate)===replicate.value)));
el('previous').addEventListener('click',()=>navigate(current-1));el('next').addEventListener('click',()=>navigate(current+1));
el('reveal').addEventListener('click',()=>{revealed.add(pairs[current].id);render()});el('showJudges').addEventListener('change',showJudgments);render();
</script></body></html>
"""


def build(base, output=None):
    data = json.dumps(collect(base), ensure_ascii=False, separators=(",", ":"))
    # JSON lives inside a script element: prevent HTML closing tags and JS separators.
    for character, escape in (("<", "\\u003c"), (">", "\\u003e"), ("&", "\\u0026"),
                              ("\u2028", "\\u2028"), ("\u2029", "\\u2029")):
        data = data.replace(character, escape)
    output = output or base / "review.html"
    output.write_text(HTML.replace("__DATA__", data), encoding="utf-8")
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", type=Path, default=ROOT / "target" / "creative-validation")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    print(build(args.base, args.output).resolve())
