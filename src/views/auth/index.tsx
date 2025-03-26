import React from "react";
import { Button, Card } from "antd";

const LoginPage = () => {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 animate-gradient-x">
      <Card className="w-96 backdrop-blur-sm bg-white/30 border border-white/10 shadow-xl">
        <div className="text-center">
          <h1 className="text-3xl font-bold mb-8">欢迎登录</h1>
          <Button
            type="primary"
            size="large"
            className="w-full h-14 text-lg transition-all duration-300 hover:scale-105 hover:shadow-lg"
            onClick={() => {
              // 处理登录逻辑
              console.log("一键登录");
            }}
          >
            一键登录
          </Button>
        </div>
      </Card>
    </div>
  );
};

export default LoginPage;
