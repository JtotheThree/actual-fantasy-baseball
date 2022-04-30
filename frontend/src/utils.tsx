export default function toTitleCase(str: string) {
  return str.replaceAll('_', ' ').replace(
    /\w\S*/g,
    function(txt) {
      return txt.charAt(0).toUpperCase() + txt.slice(1).toLowerCase();
    }
  );
}

export function convertHandednessToUI(handedness: number, switch_type: boolean) {
  let converted = "";

  if (handedness===0) {
    converted = "R"
  } else {
    converted = "L"
  }

  if (switch_type) {
    converted += " (Sw)"
  }

  return converted;
}

export function convertHandednessToGQL(handedness: string) {
  switch (handedness) {
    case "R":
      return [0, false];
    case "L":
      return [1, false];
    case "R (Sw)":
      return [0, true];
    case "L (Sw)":
      return [1, true];
  }
}