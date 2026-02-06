#!/usr/bin/env python3
"""
TransR - Plugin Testing
Testing security plugins (RustNetX and HyrOS)
"""

import sys

try:
    from transR import Feature
except ImportError:
    print("✗ Failed to import transR")
    sys.exit(1)

def separator(title):
    print("\n" + "="*60)
    print(f" {title}")
    print("="*60)

def test_port_scanner():
    separator("RUSTNETX - PORT SCANNER")
    
    f = Feature()
    
    # Test 1: Single port scan
    print("\n1. Single Port Scan:")
    print("  Testing localhost ports...")
    
    test_ports = [22, 80, 443, 3306, 8080]
    for port in test_ports:
        try:
            result = f.scan_port("127.0.0.1", port)
            status = "OPEN  " if result['is_open'] else "CLOSED"
            service = f" ({result['service']})" if result['service'] else ""
            print(f"  Port {port:5d}: {status}{service}")
        except Exception as e:
            print(f"  Port {port:5d}: ERROR - {e}")
    
    # Test 2: Quick scan
    print("\n2. Quick Scan (Common Ports):")
    print("  Scanning scanme.nmap.org...")
    print("  (This may take a few seconds)")
    
    try:
        result = f.quick_scan("scanme.nmap.org")
        
        print(f"  Host: {result['host']}")
        print(f"  Status: {'UP ✓' if result['is_alive'] else 'DOWN ✗'}")
        
        if result['is_alive']:
            print(f"  Open ports ({len(result['open_ports'])}):")
            for port in result['open_ports']:
                print(f"    - {port}")
            
            if result['services']:
                print(f"  Services:")
                for svc in result['services']:
                    print(f"    - {svc}")
        
    except Exception as e:
        print(f"  ✗ Scan failed: {e}")
        print("  (This is expected if no internet connection)")

def test_vulnerability_scanner():
    separator("HYROS - VULNERABILITY SCANNER")
    
    f = Feature()
    
    print("\n1. Vulnerability Scan:")
    print("  Scanning test target...")
    print("  (This will take a moment)")
    
    # Use a safe test target
    target = "http://testphp.vulnweb.com"
    
    try:
        results = f.vuln_scan(target)
        
        print(f"\n  Target: {target}")
        print(f"  Total checks: {len(results) if results else 'N/A'}")
        
        if not results:
            print("  ✗ No results (target may be unavailable)")
            return
        
        # Count by severity
        severity_count = {
            'Critical': 0,
            'High': 0,
            'Medium': 0,
            'Low': 0,
            'Info': 0
        }
        
        for vuln in results:
            sev = vuln['severity']
            if sev in severity_count:
                severity_count[sev] += 1
        
        print(f"\n  Vulnerabilities found:")
        print(f"    🔴 Critical: {severity_count['Critical']}")
        print(f"    🟠 High:     {severity_count['High']}")
        print(f"    🟡 Medium:   {severity_count['Medium']}")
        print(f"    🔵 Low:      {severity_count['Low']}")
        print(f"    ⚪ Info:     {severity_count['Info']}")
        
        # Show critical and high vulns
        critical_high = [v for v in results if v['severity'] in ['Critical', 'High']]
        
        if critical_high:
            print(f"\n  ⚠️  CRITICAL/HIGH VULNERABILITIES:")
            for vuln in critical_high[:5]:  # Show first 5
                icon = "🔴" if vuln['severity'] == 'Critical' else "🟠"
                print(f"    {icon} [{vuln['id']}] {vuln['name']}")
                print(f"       URL: {vuln['url']}")
                if vuln['evidence']:
                    evidence = vuln['evidence'][:80]
                    print(f"       Evidence: {evidence}...")
        
    except Exception as e:
        print(f"  ✗ Scan failed: {e}")
        print("  (This is expected if no internet connection)")

def test_combined_security_audit():
    separator("COMBINED SECURITY AUDIT")
    
    f = Feature()
    
    print("\n  Performing comprehensive security check...")
    print("  Target: localhost")
    
    # Step 1: Port scan
    print("\n  Step 1: Port Scanning...")
    try:
        scan_result = f.quick_scan("127.0.0.1")
        
        if scan_result['is_alive']:
            print(f"    ✓ Host is reachable")
            print(f"    Found {len(scan_result['open_ports'])} open ports")
            
            for port in scan_result['open_ports'][:5]:
                print(f"      - Port {port}")
        else:
            print(f"    ℹ️  Host appears to be down or filtered")
    except Exception as e:
        print(f"    ⚠️  Port scan error: {e}")
    
    # Step 2: Service identification
    print("\n  Step 2: Service Identification...")
    common_services = [
        (22, "SSH"),
        (80, "HTTP"),
        (443, "HTTPS"),
        (3306, "MySQL"),
        (5432, "PostgreSQL"),
    ]
    
    found_services = []
    for port, name in common_services:
        try:
            result = f.scan_port("127.0.0.1", port)
            if result['is_open']:
                found_services.append(f"{name} ({port})")
        except:
            pass
    
    if found_services:
        print(f"    ✓ Identified {len(found_services)} services:")
        for svc in found_services:
            print(f"      - {svc}")
    else:
        print(f"    ℹ️  No common services detected")
    
    # Step 3: Summary
    print("\n  Step 3: Security Summary...")
    print(f"    📊 Audit completed")
    print(f"    ℹ️  For full vulnerability assessment, use vuln_scan() on web targets")

def main():
    print("="*60)
    print(" TransR Security Plugins Test Suite")
    print("="*60)
    print("\n⚠️  WARNING: Only scan systems you own or have permission to test!")
    print("Unauthorized scanning may be illegal.")
    
    try:
        test_port_scanner()
        test_vulnerability_scanner()
        test_combined_security_audit()
        
        print("\n" + "="*60)
        print(" ✓ All plugin tests completed")
        print("="*60)
        
        return 0
    except Exception as e:
        print(f"\n✗ Test failed with error: {e}")
        import traceback
        traceback.print_exc()
        return 1

if __name__ == "__main__":
    sys.exit(main())
