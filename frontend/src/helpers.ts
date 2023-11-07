export function formatISODate(date: Date) {
  return [date.getFullYear(), date.getMonth() + 1, date.getDate()].map((s) => s.toString().padStart(2, "0")).join("-");
}

export function formatHHMMTime(date: Date) {
  return [date.getHours(), date.getMinutes()].map((s) => s.toString().padStart(2, "0")).join(":");
}

export function titleCase(string: string) {
  return string
    .split(" ")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}

export function formatTagName(tagName: string) {
  return titleCase(tagName.replaceAll("_", " "));
}

export function formatClearcheckMs(clearcheckMs: number) {
  let seconds = Math.ceil(clearcheckMs / 1000);

  if (seconds > 60) {
    let mins = Math.floor(seconds / 60);
    let remaining_s = seconds % 60;
    return `${mins}m ${remaining_s}s`;
  } else {
    return `${seconds}s`;
  }
}

export function formatSmm1LevelId(levelId: string) {
  let upper = levelId.toUpperCase();
  return [upper.substring(0, 4), upper.substring(4, 8), upper.substring(8, 12), upper.substring(12, 16)].join("-");
}

export function formatSmm2LevelId(levelId: string) {
  let upper = levelId.toUpperCase();
  return [upper.substring(0, 3), upper.substring(3, 6), upper.substring(6, 9)].join("-");
}
