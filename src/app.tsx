import { RouterProvider } from "react-router-dom";
import router from "./routes";

export default function () {
  return <RouterProvider router={router} />;
}
