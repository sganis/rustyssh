export default function FileBar(props) {
  const path = props.currentPath;
  return (
    <div className="pb-3 pt-3">
      <a href="#" onClick={() => props.goUp(path)}>
        Go Up
      </a>
      :&nbsp;
      <span>{path}</span>
    </div>
  );
}
