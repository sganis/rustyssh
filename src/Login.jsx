import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Login(props) {
  return (
    <div className="container">
      <form onSubmit={props.handleSubmit}>
        <fieldset disabled={props.isConnecting}>
          <div className="mb-3">
            <label>Server</label>
            <input
              type="text"
              value={props.server}
              className="form-control"
              id="server"
              placeholder="Enter remote ssh host name or IP address"
              onChange={(e) => props.setServer(e.target.value)}
            />
          </div>
          <div className="mb-3">
            <label>User</label>
            <input
              type="user"
              value={props.user}
              className="form-control"
              id="user"
              placeholder="Enter username"
              onChange={(e) => props.setUser(e.target.value)}
            />
          </div>
          <div className="mb-3">
            <label>Password</label>
            <input
              type="password"
              value={props.password}
              className="form-control"
              id="password"
              placeholder="Password"
              onChange={(e) => props.setPassword(e.target.value)}
            />
          </div>

          <div className="mb-3">
            <button type="submit" className="btn btn-primary float-end">
              Connect
            </button>
          </div>
        </fieldset>
      </form>
    </div>
  );
}
