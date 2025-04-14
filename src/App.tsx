import { useEffect, useState } from "react";
import "./App.css";
import { IoIosRefresh } from "react-icons/io";
import AddCard from "./components/AddCard";
import { Task } from "./models/task";
import TaskCard from "./components/TaskCard";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [showAll, setShowAll] = useState<Boolean>(false);
  const [rotation, setRotation] = useState<number>(45);
  const toggleShow = () => {
    setShowAll((e) => {
      return !e;
    });
  };
  const [taskList, setTaskList] = useState<Task[]>([]);
  const [dueTaskList, setDueTaskList] = useState<Task[]>([]);
  const fetchTasks = async () => {
    const result = (await invoke("get_all")) as Task[];
    const dueResult = (await invoke("get_due")) as Task[];
    setTaskList(result);
    setDueTaskList(dueResult);
  };
  useEffect(() => {
    fetchTasks();
  }, []);

  const handleRefreshBtnClick = () => {
    setRotation((prev) => prev + 360);
    fetchTasks();
  };
  return (
    <>
      <div>
        <button
          onClick={toggleShow}
          style={{ width: "90%", backgroundColor: "#505050" }}
        >
          Show {showAll ? "due" : "all"} tasks
        </button>
      </div>
      <div
        style={{
          border: "1px white dashed",
          borderRadius: "5px",
          width: "300px",
          margin: "10px",
          padding: "10px",
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            gap: "10px",
          }}
        >
          <h2>{showAll ? "All" : "Due"} Tasks</h2>
          <IoIosRefresh
            size={20}
            onClick={handleRefreshBtnClick}
            style={{
              transform: `rotate(${rotation}deg)`,
            }}
            className="refresh-btn"
          />
        </div>
        <div style={{ display: "flex", flexDirection: "column", gap: "2px" }}>
          <AddCard updateFunction={fetchTasks} />
          {(showAll ? taskList : dueTaskList).map((task) => (
            <TaskCard task={task} updateFunction={fetchTasks} />
          ))}
        </div>
      </div>
    </>
  );
}

export default App;
