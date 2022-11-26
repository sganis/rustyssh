import File from "./File";
import FileBar from "./FileBar";
//import { useState, useEffect } from "react";

export default function Files(props) {
  const files = Array.isArray(props.files)
    ? props.files.map((f, i) => (
        <File file={f} key={i} handleClick={props.handleRowClick} />
      ))
    : [];
  return (
    <div>
      <div>
        <FileBar goUp={props.goUp} currentPath={props.currentPath} />
      </div>
      <div>
        {files && (
          <table className="table table-hover">
            <tbody>{files}</tbody>
          </table>
        )}
      </div>
    </div>
  );
}
