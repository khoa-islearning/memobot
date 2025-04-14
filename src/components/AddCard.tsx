import { invoke } from "@tauri-apps/api/core";
import axios from "axios";
import { useState } from "react";
type AddCardProp = {
  updateFunction: Function;
};
const AddCard = ({ updateFunction }: AddCardProp) => {
  const [showForm, setShowForm] = useState(false);
  const [taskName, setTaskName] = useState("");
  const [taskUrl, setTaskUrl] = useState("");

  const toggleShowForm = () => {
    setShowForm((e) => {
      return !e;
    });
    setTaskName("");
    setTaskUrl("");
  };
  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    await invoke("create_task", { name: taskName, url: taskUrl });
    updateFunction();
    toggleShowForm();
  };
  return (
    <div className="task-card">
      <div style={{ cursor: "pointer" }}>
        {showForm ? (
          "New task"
        ) : (
          <p
            onClick={toggleShowForm}
            style={{ margin: "0px", padding: "0px", fontSize: "1.5rem" }}
          >
            +
          </p>
        )}
      </div>
      {showForm && (
        <div>
          <form onSubmit={handleSubmit}>
            <input
              type="text"
              placeholder="Task Name"
              value={taskName}
              onChange={(e) => setTaskName(e.target.value)}
            />
            <input
              type="text"
              placeholder="Task URL"
              value={taskUrl}
              onChange={(e) => setTaskUrl(e.target.value)}
            />
            <div
              style={{
                display: "flex",
                flexDirection: "row",
                gap: "4px",
                alignItems: "center",
                justifyContent: "center",
                paddingTop: "10px",
              }}
            >
              <button type="submit">Add</button>
              <button onClick={toggleShowForm}>Cancel</button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
};

export default AddCard;
