import {
  getWindows,
  onEvent,
  windowCaptureSend,
  windowCaptureStart,
  windowCaptureStop,
} from "@/utils/sdk";
import { Button, Col, Divider, Flex, Row, Switch } from "antd";
import { useEffect } from "react";

export default function () {
  const handleWindow = async () => {
    let list = await getWindows();
    console.log(list);
  };
  const handleSend = async () => {
    let res = await windowCaptureSend("你是谁，你想干嘛....",true);
    console.log(res);
  };
  const handleWindowChange = async (val: boolean) => {
    if (val) {
      let res = await windowCaptureStart();
      console.log(res);
    } else {
      let res = await windowCaptureStop();
      console.log(res);
    }
  };

  useEffect(() => {
    let unlisten: () => void;
    (async () => {
      unlisten = await onEvent("capture", (d) => {
        console.log(d.payload);
      });
    })();

    return () => {
      unlisten?.();
    };
  }, []);

  return (
    <div>
      <Row justify="center">
        <Col span={4}>col-4</Col>
        <Col span={4}>col-4</Col>
        <Col span={4}>col-4</Col>
        <Col span={4}>col-4</Col>
      </Row>
      <Divider />
      <Flex gap={"customize"}>
        <div>
          <label>窗口订阅</label>
          <Switch defaultChecked onChange={handleWindowChange} />
        </div>
        <div>
          <label>后台任务</label>
          <Switch defaultChecked />
        </div>
      </Flex>
      <Divider />
      <Flex gap={"customize"}>
        <Button onClick={handleWindow}>获取窗口</Button>
        <Button onClick={handleSend}>发送文本</Button>
      </Flex>
    </div>
  );
}
