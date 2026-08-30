#!/usr/bin/env bash
set -euo pipefail

task8_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
manifest="$task8_root/fixtures/manifest.json"
task8_tmp=$(mktemp -d)
trap 'rm -rf -- "$task8_tmp"' EXIT

jq -e '
  .schema_version == 1 and
  (.github.head_sha | length == 40)
' "$manifest" >/dev/null

jq -e '
  ([.github.cases[].kind] | contains(["pull_files", "review_comment", "pending_review", "check_runs"])) and
  ([.worktree.cases[]] | contains(["clean", "modified", "staged", "untracked", "conflicted", "head_diverged", "broken"])) and
  ([.execution_backend.cases[].outcome] | contains(["success", "unavailable", "backend_mismatch"])) and
  ([.faults[]] | contains(["response_reordered", "cancel_completion_race", "query_retry_exhausted", "mutation_outcome_unknown", "partial_response", "corrupt_cache", "schema_version_unknown"]))
' "$manifest" >/dev/null

if rg -ni 'authorization|access[_ -]?token|refresh[_ -]?token|client[_ -]?secret' "$task8_root/fixtures"; then
  printf '%s\n' 'fixtureにsecretらしいfieldが含まれています' >&2
  exit 1
fi

repo="$task8_tmp/repo"
git init -q -b main "$repo"
git -C "$repo" config user.name "TASK-8 fixture"
git -C "$repo" config user.email "task-8@example.invalid"
printf 'base\n' >"$repo/state.txt"
git -C "$repo" add state.txt
git -C "$repo" commit -qm base
test -z "$(git -C "$repo" status --porcelain=v1)"

printf 'modified\n' >"$repo/state.txt"
git -C "$repo" status --porcelain=v1 | grep -Fqx ' M state.txt'
git -C "$repo" add state.txt
git -C "$repo" status --porcelain=v1 | grep -Fqx 'M  state.txt'
git -C "$repo" commit -qm modified

printf 'untracked\n' >"$repo/untracked.txt"
git -C "$repo" status --porcelain=v1 | grep -Fqx '?? untracked.txt'
rm "$repo/untracked.txt"

git -C "$repo" switch -qc other
printf 'other\n' >"$repo/state.txt"
git -C "$repo" commit -qam other
git -C "$repo" switch -q main
printf 'main\n' >"$repo/state.txt"
git -C "$repo" commit -qam main
if git -C "$repo" merge other >/dev/null 2>&1; then
  printf '%s\n' 'conflict fixtureの生成に失敗しました' >&2
  exit 1
fi
git -C "$repo" status --porcelain=v1 | grep -Fqx 'UU state.txt'
git -C "$repo" merge --abort

git -C "$repo" branch remote-main HEAD~1
test "$(git -C "$repo" rev-list --count remote-main..main)" -eq 1

cp "$repo/.git/HEAD" "$task8_tmp/HEAD.saved"
printf 'ref: refs/heads/does-not-exist\n' >"$repo/.git/HEAD"
if git -C "$repo" rev-parse --verify HEAD >/dev/null 2>&1; then
  printf '%s\n' 'broken HEAD fixtureが成功扱いになりました' >&2
  exit 1
fi
cp "$task8_tmp/HEAD.saved" "$repo/.git/HEAD"

printf '%s\n' 'TASK-8 fixture contract and worktree states: OK'
