import React, { SyntheticEvent, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

type HandleSubmitFunction = (e: SyntheticEvent) => void;

export default function App(): React.ReactElement {
  const messageRef = useRef<HTMLSpanElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit: HandleSubmitFunction = (e) => {
    e.preventDefault();
    const msgExists = messageRef.current;

    if (msgExists) {
      messageRef.current.innerText = "";
    }

    const name = inputRef.current?.value;
    if (name) {
      invoke<string>("greet", { name }).then((greeting: string) => {
        if (msgExists) {
          messageRef.current.innerText = greeting;
        }
        if (inputRef.current?.value) {
          inputRef.current.value = "";
        }
      });
    }
  };

  return (
    <div className="layout">
      <div className="inner-layout">
        <form id="form" onSubmit={handleSubmit}>
          <label htmlFor="first-input">Enter your name: </label>
          <input ref={inputRef} id="first-input" type="text" />
          <button className="ok-btn" type="submit">
            OK
          </button>
          <div>
            Message: <span ref={messageRef} />
          </div>
        </form>
      </div>
    </div>
  );
}
