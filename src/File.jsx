import folderIcon from "./assets/folder.png";
import fileIcon from "./assets/file.png";

export default function File(props) {
  //   function get_path(path) {
  //     get_files(path);
  //   }

  const f = props.file;

  return (
    <tr onClick={() => props.handleClick(f.path)} style={{ cursor: "pointer" }}>
      <td width="50px">
        <img src={f.filetype == "dir" ? folderIcon : fileIcon} width="28" />
      </td>
      <td>{f.name}</td>
      <td>{f.size}</td>
      <td>{f.modified}</td>
    </tr>
  );
}
