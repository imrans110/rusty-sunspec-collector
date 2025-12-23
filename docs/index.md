---
layout: default
title: Rusty SunSpec Collector Docs
permalink: /
---

<div class="doc-hero">
  <p class="doc-kicker">Rusty SunSpec Collector Docs</p>
  <h1>Field-ready docs for building and operating the collector.</h1>
  <p>Quick links for build, runtime operations, and maintenance. Use the plan for the latest execution status.</p>
</div>

<div class="doc-grid">
  <a class="doc-card" href="{{ '/build' | relative_url }}">
    <h3>Build, Test, Run</h3>
    <p>Local builds, cross-compilation, and test commands.</p>
  </a>
  <a class="doc-card" href="{{ '/ops' | relative_url }}">
    <h3>Runtime Operations</h3>
    <p>systemd setup, log access, and buffer location guidance.</p>
  </a>
  <a class="doc-card" href="{{ '/buffer_maintenance' | relative_url }}">
    <h3>Buffer Maintenance</h3>
    <p>VACUUM and WAL checkpoint strategies.</p>
  </a>
  <a class="doc-card" href="{{ '/plan' | relative_url }}">
    <h3>Execution Plan</h3>
    <p>Current tracker and project plan details.</p>
  </a>
  <a class="doc-card" href="{{ '/config.example.toml' | relative_url }}">
    <h3>Config Example</h3>
    <p>Starter TOML config with defaults and overrides.</p>
  </a>
</div>

<div class="doc-fallback">
  <h2>Direct repo links</h2>
  <ul>
    <li><a href="build.md">Build, Test, Run</a></li>
    <li><a href="ops.md">Runtime Operations</a></li>
    <li><a href="buffer_maintenance.md">Buffer Maintenance</a></li>
    <li><a href="plan.md">Execution Plan</a></li>
    <li><a href="config.example.toml">Config Example</a></li>
  </ul>
</div>

<div class="doc-footer">
  Docs site is published via GitHub Pages from the <code>docs/</code> folder.
</div>
