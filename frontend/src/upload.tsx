import React, { useState } from "react";
import {
  Upload,
  Button,
  Select,
  Slider,
  InputNumber,
  Card,
  message,
  Row,
  Col,
} from "antd";
import { InboxOutlined, DownloadOutlined } from "@ant-design/icons";
import axios from "axios";

const { Dragger } = Upload;

const UploadPage: React.FC = () => {
  const [pdfFile, setPdfFile] = useState<File | null>(null);
  const [format, setFormat] = useState<string>("png");
  const [dpi, setDpi] = useState<number>(150);
  const [loading, setLoading] = useState<boolean>(false);


  const props = {
    name: "pdf",
    multiple: false,
    accept: ".pdf",
    beforeUpload(file: File) {
      setPdfFile(file as File);
      return false; // avoid auto upload
    },
  };

  const handleConvert = async () => {
    if (!pdfFile) {
      message.error("Please upload a PDF file first.");
      return;
    }

    const formData = new FormData();
    formData.append("pdf", pdfFile);

    setLoading(true);
    try {
      const response = await axios.post(
        `http://127.0.0.1:5800/convert?format=${format}&dpi=${dpi}`,
        formData,
        {
          responseType: "blob",
          headers: { "Content-Type": "multipart/form-data" },
        }
      );

      // Download ZIP
      const blob = new Blob([response.data], { type: "application/zip" });
      const url = window.URL.createObjectURL(blob);

      const link = document.createElement("a");
      link.href = url;
      link.download = `converted-${Date.now()}.zip`;
      link.click();

      window.URL.revokeObjectURL(url);

      message.success("PDF successfully converted!");
    } catch (err) {
      console.error(err);
      message.error("Error converting PDF.");
    }

    setLoading(false);
  };

  return (
    <Row justify="center" style={{ marginTop: 40 }}>
      <Col xs={22} sm={18} md={12} lg={10}>
        <Card
          title="📄➡️🖼️ PDF → Image Converter"
          bordered={false}
          style={{
            borderRadius: 12,
            boxShadow: "0 4px 18px rgba(0,0,0,0.08)",
          }}
        >
          <Dragger {...props} style={{ padding: "20px" }}>
            <p className="ant-upload-drag-icon">
              <InboxOutlined />
            </p>
            <p className="ant-upload-text">Click or drag PDF file to upload</p>
            <p className="ant-upload-hint">Only PDF files are supported</p>
          </Dragger>

          <br />

          <h4>Image Format</h4>
          <Select
            value={format}
            onChange={setFormat}
            style={{ width: "100%" }}
            options={[
              { value: "png", label: "PNG" },
              { value: "jpg", label: "JPG" },
            ]}
          />

          <br />
          <br />

          <h4>DPI (Image Quality)</h4>
          <Row>
            <Col span={16}>
              <Slider
                min={72}
                max={300}
                value={dpi}
                onChange={(value) => setDpi(value as number)}
              />
            </Col>
            <Col span={6} offset={2}>
              <InputNumber
                min={72}
                max={300}
                value={dpi}
                onChange={(value) => {
                  if (value !== null) setDpi(value);
                }}
                style={{ width: "100%" }}
              />
            </Col>
          </Row>

          <br />

          <Button
            type="primary"
            icon={<DownloadOutlined />}
            block
            size="large"
            loading={loading}
            onClick={handleConvert}
            disabled={!pdfFile}
          >
            Convert & Download ZIP
          </Button>
        </Card>
      </Col>
    </Row>
  );
};

export default UploadPage;
