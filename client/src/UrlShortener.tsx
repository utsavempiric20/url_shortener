/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useEffect, useState } from "react";
import axios from "axios";
import {
  Box,
  TextField,
  Button,
  Typography,
  Alert,
  Card,
  CardContent,
  Divider,
} from "@mui/material";

// API response types
interface CreateLinkResponse {
  slug: string;
}
interface GetCountResponse {
  clicks: number;
}

const UrlShortener: React.FC = () => {
  const [longUrl, setLongUrl] = useState("");
  const [shortUrl, setShortUrl] = useState("");
  const [error, setError] = useState("");

  const [countSlug, setCountSlug] = useState("");
  const [clickCount, setClickCount] = useState<number | null>(null);
  const [countError, setCountError] = useState("");

  const [copySuccess, setCopySuccess] = useState("");

  const apiBase = import.meta.env.VITE_REACT_APP_API_BASE_URL || "";

  useEffect(() => {
    if (copySuccess) {
      const timer = setTimeout(() => setCopySuccess(""), 3000);
      return () => clearTimeout(timer);
    }
  }, [copySuccess]);

  const handleShorten = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    try {
      const { data } = await axios.post<CreateLinkResponse>(
        `${apiBase}/api/links`,
        { url: longUrl }
      );

      setShortUrl(
        `${import.meta.env.VITE_REACT_APP_API_BASE_URL}/${data.slug}`
      );
      setLongUrl("");
    } catch (err: any) {
      setError(
        err.response?.data?.message ||
          err.message ||
          "Failed to create short link"
      );
    }
  };

  const handleCopySlug = () => {
    if (!shortUrl) return;
    navigator.clipboard.writeText(shortUrl).then(
      () => setCopySuccess("Slug copied to clipboard!"),
      () => setCopySuccess("Failed to copy slug")
    );
  };

  const handleGetCount = async (e: React.FormEvent) => {
    e.preventDefault();
    setCountError("");
    setClickCount(null);
    let trimmed = countSlug.trim();
    try {
      const urlObj = new URL(
        trimmed.includes("://")
          ? trimmed
          : `${window.location.origin}/${trimmed}`
      );
      trimmed = urlObj.pathname.replace(/^\//, "");
    } catch {
      // if invalid URL, use as-is
    }

    try {
      const { data } = await axios.get<GetCountResponse>(
        `${apiBase}/api/getcount/${trimmed}`
      );
      setClickCount(data.clicks);
    } catch (err: any) {
      setCountError(
        err.response?.data?.message || err.message || "Failed to fetch count"
      );
    }
  };

  return (
    <Card elevation={3} sx={{ p: 2, width: "100%" }}>
      <CardContent>
        <Typography variant="h4" gutterBottom>
          URL Shortener
        </Typography>

        <Box component="form" onSubmit={handleShorten} sx={{ mb: 3 }}>
          <TextField
            fullWidth
            label="Enter URL"
            placeholder="https://example.com"
            value={longUrl}
            onChange={(e: {
              target: { value: React.SetStateAction<string> };
            }) => setLongUrl(e.target.value)}
            required
            sx={{ mb: 2 }}
          />
          <Button variant="contained" type="submit" fullWidth>
            Shorten URL
          </Button>
        </Box>

        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {error}
          </Alert>
        )}

        {shortUrl && (
          <Box sx={{ mb: 3 }}>
            <Typography variant="subtitle1">Your short link:</Typography>
            <Box sx={{ display: "flex", alignItems: "center", gap: 1, mt: 1 }}>
              <Button
                component="a"
                href={shortUrl}
                target="_blank"
                rel="noopener noreferrer"
                variant="outlined"
                onClick={handleCopySlug}
              >
                {shortUrl}
              </Button>
              <Button variant="contained" onClick={handleCopySlug}>
                Copy Slug
              </Button>
            </Box>
            {copySuccess && (
              <Typography variant="body2" color="success.main" sx={{ mt: 1 }}>
                {copySuccess}
              </Typography>
            )}
          </Box>
        )}

        <Divider sx={{ my: 3 }} />

        <Typography variant="h5" gutterBottom>
          Get Click Count
        </Typography>
        <Box component="form" onSubmit={handleGetCount} sx={{ mb: 2 }}>
          <TextField
            fullWidth
            label="Enter Slug"
            placeholder="abc123"
            value={countSlug}
            onChange={(e: {
              target: { value: React.SetStateAction<string> };
            }) => setCountSlug(e.target.value)}
            required
            sx={{ mb: 2 }}
          />
          <Button variant="contained" type="submit" fullWidth>
            Get Count
          </Button>
        </Box>

        {countError && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {countError}
          </Alert>
        )}
        {clickCount !== null && (
          <Typography>
            Clicks for <strong>{countSlug}</strong>: {clickCount}
          </Typography>
        )}
      </CardContent>
    </Card>
  );
};

export default UrlShortener;
