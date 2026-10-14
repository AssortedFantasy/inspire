const el = id => document.getElementById(id);
const buttons = [...document.querySelectorAll('.actions button')];
let sample;
let busy = true;
let saved = false;
let lastSubmission;
let retry;

function lock(value) {
  busy = value;
  buttons.forEach(button => { button.disabled = value || !sample || saved; });
  el('family').disabled = value;
  el('notes').disabled = value || saved;
}

function status(message, error = false, action = null) {
  el('status').textContent = message;
  el('status').parentElement.classList.toggle('error', error);
  el('retry').hidden = !action;
  retry = action;
}

async function api(path, options) {
  const response = await fetch(path, options);
  const body = await response.json();
  if (!response.ok) throw new Error(body.error || 'Request failed');
  return body;
}

async function next(message = '') {
  lock(true);
  el('retry').hidden = true;
  document.querySelector('.sample').setAttribute('aria-busy', 'true');
  try {
    const incoming = await api(`/api/sample?family=${encodeURIComponent(el('family').value)}`);
    sample = incoming;
    saved = false;
    lastSubmission = null;
    el('source').textContent = `${sample.path} · ${sample.kind === 'record' ? 'Record' : 'Generated'}`;
    el('text').textContent = sample.text;
    el('notes').value = '';
    status(message || '');
    window.scrollTo(0, 0);
  } catch (error) {
    status(`${message ? message + ' ' : ''}${error.message}`, true, () => next(message));
  } finally {
    document.querySelector('.sample').setAttribute('aria-busy', 'false');
    lock(false);
  }
}

async function submit(rating) {
  if (busy || saved || !sample) return;
  // Keep the exact request on retry in case the response was lost after the write.
  lastSubmission ??= { id: sample.id, rating, notes: el('notes').value };
  lock(true);
  status('Saving…');
  try {
    const result = await api('/api/feedback', {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(lastSubmission),
    });
    saved = true;
    el('count').textContent = `${result.reviewed} reviewed this session`;
    await next(`${lastSubmission.rating} saved.`);
  } catch (error) {
    status(error.message, true, () => submit(rating));
  } finally {
    lock(false);
    if (lastSubmission && !saved) {
      // Resolve an uncertain save before accepting a different rating or notes.
      buttons.forEach(button => { button.disabled = true; });
      el('notes').disabled = true;
      el('family').disabled = true;
    }
  }
}

el('review').addEventListener('submit', event => {
  event.preventDefault();
  if (event.submitter) submit(event.submitter.value);
});
el('family').addEventListener('change', () => next());
el('retry').addEventListener('click', () => retry?.());

(async () => {
  try {
    const config = await api('/api/config');
    config.families.forEach(family => el('family').add(new Option(family, family)));
    el('file').textContent = config.feedback;
    el('count').textContent = `${config.reviewed} reviewed this session`;
    await next();
  } catch (error) {
    status(error.message, true, () => location.reload());
  }
})();
