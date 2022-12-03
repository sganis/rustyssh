export const getParent = (path) => {
  let index = path.lastIndexOf("/");
  if (index < 0) return "/";
  index = index == 0 ? 1 : index;
  let parent = path.substring(0, index);
  console.log("path  : ", path);
  console.log("parent: ", parent);
  return parent;
};

export function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
