export type AociPlatform = "win32" | "linux" | "darwin";
export type AociArchitecture = "x64" | "arm64";

export type AociReleaseAsset = {
  archive: string;
  sha256: string;
  executable: "aoci" | "aoci.exe";
  url: string;
};

export const CODEGRAPH_RELEASE = Object.freeze({
  package: "@colbymchenry/codegraph",
  version: "1.6.0",
  integrity: "sha512-nCN40MqmYxF7gH1QTKqlxJ1d2mzwhw3fzSdGV2wjnQKsymjM3JZnH/5rpGqhBkcUEom0qWq0WjDRvOZh2t8mFA==",
});

const AOCI_VERSION = "0.1.0-rc12";
const releaseBase = "https://github.com/aoci-spec/aoci-code/releases/download/v0.1.0-rc12";

function asset(archive: string, sha256: string, executable: "aoci" | "aoci.exe"): AociReleaseAsset {
  return { archive, sha256, executable, url: releaseBase + "/" + archive };
}

export const AOCI_RELEASE = Object.freeze({
  version: AOCI_VERSION,
  tag: "v" + AOCI_VERSION,
  assets: Object.freeze({
    "linux-x64": asset("aoci_0.1.0-rc12_linux_amd64.tar.gz", "eeacdbbdb84cbd3e36ab50e9404b581a1acd66b87f0a7e81447357fce61ad31b", "aoci"),
    "linux-arm64": asset("aoci_0.1.0-rc12_linux_arm64.tar.gz", "75f12d01d8f024164775e506e47e5e00d1beaac157fb95e56524d2b16a2a209e", "aoci"),
    "darwin-x64": asset("aoci_0.1.0-rc12_darwin_amd64.tar.gz", "641b059961de17f47019da21b3b44a8a95a35c4cca1be32090301a6e082902d4", "aoci"),
    "darwin-arm64": asset("aoci_0.1.0-rc12_darwin_arm64.tar.gz", "87178f236ec8153e2bf0a792682f4c1d68256ab2a5c290b95bc035852be89cfa", "aoci"),
    "win32-x64": asset("aoci_0.1.0-rc12_windows_amd64.zip", "3b6a80f66ab2411b1cafa8142275622c862bae0a55e6efb9f27bd589faf790da", "aoci.exe"),
    "win32-arm64": asset("aoci_0.1.0-rc12_windows_arm64.zip", "c529ff3f324c4432bbe037a8c86f8b1955e3f52b46d98b4edc950f0e8f8bead5", "aoci.exe"),
  }),
});

export function selectAociAsset(platform: string, architecture: string): AociReleaseAsset {
  const value = AOCI_RELEASE.assets[(platform + "-" + architecture) as keyof typeof AOCI_RELEASE.assets];
  if (!value) throw new Error("unsupported AOCI platform: " + platform + "/" + architecture);
  return value;
}
