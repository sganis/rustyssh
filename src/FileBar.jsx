export default function FileBar(props) {
  const path = props.currentPath;
  return (
    <div>
      <a href="#" onClick={() => props.goUp(path)}>
        Go Up
      </a>
      :&nbsp;
      <span>{path}</span>
    </div>
  );
}
