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

const ClearConditionTexts = new Map([
  [0, "None"],
  [1, "Reach the goal without landing after leaving the ground."],
  [2, "Reach the goal after defeating (n) Mechakoopa(s)."],
  [3, "Reach the goal after defeating (n) Cheep Cheep(s)."],
  [4, "Reach the goal without taking damage."],
  [5, "Reach the goal as Boomerang Mario."],
  [6, "Reach the goal while wearing a Shoe."],
  [7, "Reach the goal as Fire Mario."],
  [8, "Reach the goal as Frog Mario."],
  [9, "Reach the goal after defeating (n) Larry(s)."],
  [10, "Reach the goal as Raccoon Mario."],
  [11, "Reach the goal after defeating (n) Blooper(s)."],
  [12, "Reach the goal as Propeller Mario."],
  [13, "Reach the goal while wearing a Propeller Box."],
  [14, "Reach the goal after defeating (n) Spike(s)."],
  [15, "Reach the goal after defeating (n) Boom Boom(s)."],
  [16, "Reach the goal while holding a Koopa Shell."],
  [17, "Reach the goal after defeating (n) Porcupuffer(s)."],
  [18, "Reach the goal after defeating (n) Charvaargh(s)."],
  [19, "Reach the goal after defeating (n) Bullet Bill(s)."],
  [20, "Reach the goal after defeating (n) Bully/Bullies."],
  [21, "Reach the goal while wearing a Goomba Mask."],
  [22, "Reach the goal after defeating (n) Hop-Chops."],
  [23, "Reach the goal while holding a Red POW Block. OR Reach the goal after activating (n) Red POW Block(s)."],
  [24, "Reach the goal after defeating (n) Bob-omb(s)."],
  [25, "Reach the goal after defeating (n) Spiny/Spinies."],
  [26, "Reach the goal after defeating (n) Bowser(s)/Meowser(s)."],
  [27, "Reach the goal after defeating (n) Ant Trooper(s)."],
  [28, "Reach the goal on a Lakitu's Cloud."],
  [29, "Reach the goal after defeating (n) Boo(s)."],
  [30, "Reach the goal after defeating (n) Roy(s)."],
  [31, "Reach the goal while holding a Trampoline."],
  [32, "Reach the goal after defeating (n) Morton(s)."],
  [33, "Reach the goal after defeating (n) Fish Bone(s)."],
  [34, "Reach the goal after defeating (n) Monty Mole(s)."],
  [35, "Reach the goal after picking up (n) 1-Up Mushroom(s)."],
  [36, "Reach the goal after defeating (n) Hammer Bro(s)."],
  [37, "Reach the goal after hitting (n) P Switch(es). OR Reach the goal while holding a P Switch."],
  [38, "Reach the goal after activating (n) POW Block(s). OR Reach the goal while holding a POW Block."],
  [39, "Reach the goal after defeating (n) Angry Sun(s)."],
  [40, "Reach the goal after defeating (n) Pokey(s)."],
  [41, "Reach the goal as Superball Mario."],
  [42, "Reach the goal after defeating (n) Pom Pom(s)."],
  [43, "Reach the goal after defeating (n) Peepa(s)."],
  [44, "Reach the goal after defeating (n) Lakitu(s)."],
  [45, "Reach the goal after defeating (n) Lemmy(s)."],
  [46, "Reach the goal after defeating (n) Lava Bubble(s)."],
  [47, "Reach the goal while wearing a Bullet Bill Mask."],
  [48, "Reach the goal as Big Mario."],
  [49, "Reach the goal as Cat Mario."],
  [50, "Reach the goal after defeating (n) Goomba(s)/Galoomba(s)."],
  [51, "Reach the goal after defeating (n) Thwomp(s)."],
  [52, "Reach the goal after defeating (n) Iggy(s)."],
  [53, "Reach the goal while wearing a Dry Bones Shell."],
  [54, "Reach the goal after defeating (n) Sledge Bro(s)."],
  [55, "Reach the goal after defeating (n) Rocky Wrench(es)."],
  [56, "Reach the goal after grabbing (n) 50-Coin(s)."],
  [57, "Reach the goal as Flying Squirrel Mario."],
  [58, "Reach the goal as Buzzy Mario."],
  [59, "Reach the goal as Builder Mario."],
  [60, "Reach the goal as Cape Mario."],
  [61, "Reach the goal after defeating (n) Wendy(s)."],
  [62, "Reach the goal while wearing a Cannon Box."],
  [63, "Reach the goal as Link."],
  [64, "Reach the goal while you have Super Star invincibility."],
  [65, "Reach the goal after defeating (n) Goombrat(s)/Goombud(s)."],
  [66, "Reach the goal after grabbing (n) 10-Coin(s)."],
  [67, "Reach the goal after defeating (n) Buzzy Beetle(s)."],
  [68, "Reach the goal after defeating (n) Bowser Jr.(s)."],
  [69, "Reach the goal after defeating (n) Koopa Troopa(s)."],
  [70, "Reach the goal after defeating (n) Chain Chomp(s)."],
  [71, "Reach the goal after defeating (n) Muncher(s)."],
  [72, "Reach the goal after defeating (n) Wiggler(s)."],
  [73, "Reach the goal as SMB2 Mario."],
  [74, "Reach the goal in a Koopa Clown Car/Junior Clown Car."],
  [75, "Reach the goal as Spiny Mario."],
  [76, "Reach the goal in a Koopa Troopa Car."],
  [77, "Reach the goal after defeating (n) Piranha Plant(s)/Jumping Piranha Plant(s)."],
  [78, "Reach the goal after defeating (n) Dry Bones."],
  [79, "Reach the goal after defeating (n) Stingby/Stingbies."],
  [80, "Reach the goal after defeating (n) Piranha Creeper(s)."],
  [81, "Reach the goal after defeating (n) Fire Piranha Plant(s)."],
  [82, "Reach the goal after breaking (n) Crates(s)."],
  [83, "Reach the goal after defeating (n) Ludwig(s)."],
  [84, "Reach the goal as Super Mario."],
  [85, "Reach the goal after defeating (n) Skipsqueak(s)."],
  [86, "Reach the goal after grabbing (n) Coin(s)."],
  [87, "Reach the goal after defeating (n) Magikoopa(s)."],
  [88, "Reach the goal after grabbing (n) 30-Coin(s)."],
  [89, "Reach the goal as Balloon Mario."],
  [90, "Reach the goal while wearing a Red POW Box."],
  [91, "Reach the Goal while riding Yoshi."],
  [92, "Reach the goal after defeating (n) Spike Top(s)."],
  [93, "Reach the goal after defeating (n) Banzai Bill(s)."],
]);

export function formatClearCondition(ccId: number, ccMagnitude?: number) {
  let ccText = ClearConditionTexts.get(ccId)!;

  if (!ccMagnitude) {
    return ccText;
  }

  return ccText.replaceAll("(n)", ccMagnitude.toString()).replaceAll("(s)", ccMagnitude > 1 ? "s" : "");
}
