#!/usr/bin/env bash
set -euo pipefail

task4_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
task4_tmp=$(mktemp -d)
trap 'rm -rf -- "$task4_tmp"' EXIT

repo="$task4_tmp/repo"
mkdir -p "$repo/src" "$repo/assets"
git -C "$repo" init -q
git -C "$repo" config user.name "TASK-4 fixture"
git -C "$repo" config user.email "task-4@example.invalid"

printf 'one\nold-left\ncontext\ndeleted-left\ntail\n' >"$repo/src/lines.txt"
for number in $(seq 1 20); do
  printf 'line %s\n' "$number"
done >"$repo/src/rename-old.txt"
printf 'value = 1\n' >"$repo/src/whitespace.txt"
printf 'without-final-newline' >"$repo/src/no-newline.txt"
printf 'delete me\n' >"$repo/src/deleted.txt"
printf '\x00\x01\x02\x03' >"$repo/assets/binary.dat"
git -C "$repo" add .
git -C "$repo" commit -qm base
base_sha=$(git -C "$repo" rev-parse HEAD)

git -C "$repo" switch -qc feature
printf 'one\nnew-right\ncontext\nadded-right\ntail\n' >"$repo/src/lines.txt"
git -C "$repo" mv src/rename-old.txt src/rename-new.txt
sed -i '7s/line 7/line seven/' "$repo/src/rename-new.txt"
printf 'value    =    1\n' >"$repo/src/whitespace.txt"
printf 'with-final-newline\n' >"$repo/src/no-newline.txt"
rm "$repo/src/deleted.txt"
printf 'new file\n' >"$repo/src/added.txt"
printf '\x00\x01\x09\x03' >"$repo/assets/binary.dat"
git -C "$repo" add -A
git -C "$repo" commit -qm feature
head_sha=$(git -C "$repo" rev-parse HEAD)

git -C "$repo" switch -qc base-advanced "$base_sha"
printf 'base branch only\n' >"$repo/src/base-only.txt"
git -C "$repo" add src/base-only.txt
git -C "$repo" commit -qm base-advanced
git -C "$repo" switch -q feature

git -C "$repo" diff --find-renames --binary --no-ext-diff --unified=3 \
  "$base_sha" "$head_sha" >"$task4_tmp/local.diff"

git -C "$repo" diff --find-renames --name-status "$base_sha" "$head_sha" |
  grep -Eq '^R[0-9]+[[:space:]]+src/rename-old.txt[[:space:]]+src/rename-new.txt$'
grep -q '^GIT binary patch$' "$task4_tmp/local.diff"
grep -q '^\\ No newline at end of file$' "$task4_tmp/local.diff"

if git -C "$repo" diff --quiet "$base_sha" "$head_sha" -- src/whitespace.txt; then
  printf '%s\n' '通常diffでwhitespace差分を検出できませんでした' >&2
  exit 1
fi
git -C "$repo" diff --quiet -w "$base_sha" "$head_sha" -- src/whitespace.txt

if git -C "$repo" diff --name-only base-advanced...feature | grep -q '^src/base-only.txt$'; then
  printf '%s\n' 'three-dot diffへbase側だけの変更が混入しました' >&2
  exit 1
fi
git -C "$repo" diff --name-only base-advanced..feature | grep -q '^src/base-only.txt$'

git -C "$repo" diff --unified=3 "$base_sha" "$head_sha" -- src/lines.txt |
  awk '
    /^@@ / {
      split(substr($2, 2), old_range, ",")
      split(substr($3, 2), new_range, ",")
      old_line = old_range[1]
      new_line = new_range[1]
      in_hunk = 1
      next
    }
    /^diff --git / { in_hunk = 0 }
    in_hunk && /^ / {
      printf "context\t%d\t%d\t%s\n", old_line, new_line, substr($0, 2)
      old_line++
      new_line++
      next
    }
    in_hunk && /^-/ {
      printf "deletion\t%d\t-\t%s\n", old_line, substr($0, 2)
      old_line++
      next
    }
    in_hunk && /^\+/ {
      printf "addition\t-\t%d\t%s\n", new_line, substr($0, 2)
      new_line++
    }
  ' >"$task4_tmp/coordinates.tsv"

grep -Fqx $'deletion\t2\t-\told-left' "$task4_tmp/coordinates.tsv"
grep -Fqx $'addition\t-\t2\tnew-right' "$task4_tmp/coordinates.tsv"
grep -Fqx $'deletion\t4\t-\tdeleted-left' "$task4_tmp/coordinates.tsv"
grep -Fqx $'addition\t-\t4\tadded-right' "$task4_tmp/coordinates.tsv"

jq -e '
  length == 3 and
  any(.[]; .status == "renamed" and has("previous_filename")) and
  any(.[]; .filename == "assets/binary.dat" and (has("patch") | not))
' "$task4_root/fixtures/github-pr-files.json" >/dev/null

jq -e '
  .create_pending_review.body as $review |
  ($review | has("event") | not) and
  ($review.commit_id | length == 40) and
  all($review.comments[] | select(.subject_type != "file");
    has("line") and (.side == "LEFT" or .side == "RIGHT") and (has("position") | not)) and
  all($review.comments[] | select(.subject_type == "file");
    (has("line") | not) and (has("side") | not)) and
  (.submit_pending_review.body.event == "COMMENT") and
  (.delete_pending_review.method == "DELETE")
' "$task4_root/fixtures/review-payloads.json" >/dev/null

printf '%s\n' "TASK-4 diff/review API fixture verification: OK"
