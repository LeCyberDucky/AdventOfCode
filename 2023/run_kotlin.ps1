param(
    # Path to the source code that should be compiled and exectued
    [Parameter(Mandatory = $true)]
    [string] $source_path,
    # Whether or not to compile to a native binary
    [switch] $native
)


$directory = Split-Path $source_path -Parent
$source_file = Split-Path $source_path -Leaf
$file_name = Split-Path $source_path -LeafBase

# Temporarily switch to the source directory during execution, so relative paths used in the code still work
Push-Location $directory

if ($native) {
    $exe_file = ".\" + $file_name + ".exe"
    kotlinc-native $source_file -o $file_name
    Invoke-Expression -Command "$exe_file"
} else {
    $jar_file = $file_name + ".jar"
    kotlinc $source_file -include-runtime -d $jar_file
    java -jar $jar_file
}

Pop-Location